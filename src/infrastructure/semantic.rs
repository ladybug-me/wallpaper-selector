use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::infrastructure::executable;

pub const QUERY_DEBOUNCE: Duration = Duration::from_millis(350);
const SCORE_WINDOW: f32 = 0.022;
const MIN_SCORE_PROMINENCE: f32 = 0.015;
const NEGATIVE_WEIGHT: f32 = 0.5;
const MIN_RESULTS: usize = 0;

#[cfg(unix)]
const HELPER_SHUTDOWN_SIGNAL: libc::c_int = libc::SIGKILL;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticResult {
    pub generation: u64,
    pub keys: Vec<String>,
    pub exclusions: Vec<String>,
    pub query_ms: f64,
    pub search_ms: f64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticPaths {
    pub bin: PathBuf,
    pub manifest: PathBuf,
    pub index: PathBuf,
    pub runtime: PathBuf,
    fixed_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticTooling {
    pub bin: PathBuf,
    pub runtime: PathBuf,
}

pub struct SemanticService {
    tx: Sender<ServiceCommand>,
    processes: ProcessSlot,
    stopped: Arc<AtomicBool>,
}

enum ServiceCommand {
    Query(Query),
    Stop,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Query {
    generation: u64,
    #[serde(rename = "query")]
    text: String,
    negative_query: Option<String>,
    negative_weight: f32,
    top_k: usize,
    score_window: f32,
    min_score_prominence: f32,
    max_results: usize,
    min_results: usize,
    #[serde(skip_serializing)]
    exclusions: Vec<String>,
    #[serde(skip_serializing)]
    debounce: Duration,
}

#[allow(clippy::struct_field_names)]
pub struct SemanticQuery {
    pub generation: u64,
    pub text: String,
    pub negative_query: Option<String>,
    pub exclusions: Vec<String>,
    pub top_k: usize,
    pub max_results: usize,
    pub debounce: Duration,
}

impl From<SemanticQuery> for Query {
    fn from(query: SemanticQuery) -> Self {
        Self {
            generation: query.generation,
            text: query.text,
            negative_query: query.negative_query,
            negative_weight: NEGATIVE_WEIGHT,
            top_k: query.top_k,
            score_window: SCORE_WINDOW,
            min_score_prominence: MIN_SCORE_PROMINENCE,
            max_results: query.max_results,
            min_results: MIN_RESULTS,
            exclusions: query.exclusions,
            debounce: query.debounce,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    generation: u64,
    query_ms: f64,
    search_ms: f64,
    matches: Vec<Match>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct Match {
    key: String,
}

struct SemanticChild {
    child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    _active: ActiveProcess,
}

#[derive(Clone, Default)]
struct ProcessSlot(Arc<AtomicU32>);

struct ActiveProcess {
    pid: u32,
    slot: ProcessSlot,
}

impl ProcessSlot {
    fn register(&self, pid: u32) -> ActiveProcess {
        self.0.store(pid, Ordering::Release);
        ActiveProcess { pid, slot: self.clone() }
    }

    fn terminate(&self) {
        let pid = self.0.swap(0, Ordering::AcqRel);
        if pid == 0 {
            return;
        }
        #[cfg(unix)]
        unsafe {
            libc::kill(pid as libc::pid_t, HELPER_SHUTDOWN_SIGNAL);
        }
    }
}

impl Drop for ActiveProcess {
    fn drop(&mut self) {
        let _ = self.slot.0.compare_exchange(self.pid, 0, Ordering::AcqRel, Ordering::Acquire);
    }
}

impl SemanticPaths {
    pub fn discover(
        cache_dir: &str,
        selected_manifest: &str,
        profile: &str,
    ) -> Result<Self, String> {
        let home = env_path_with_fallback("SKWD_LENS_HOME", "SKWD_SEMANTIC_HOME");
        let explicit_manifest =
            env_path_with_fallback("SKWD_LENS_MANIFEST", "SKWD_SEMANTIC_MANIFEST").or_else(|| {
                (!selected_manifest.trim().is_empty()).then(|| PathBuf::from(selected_manifest))
            });
        let custom_manifest = explicit_manifest.is_some();
        let home_manifest = home.as_ref().map(|root| root.join("semantic-pack.json"));
        let manifest = explicit_manifest
            .or(home_manifest.filter(|path| path.is_file()))
            .or_else(|| installed_manifest(cache_dir))
            .ok_or_else(|| String::from("semantic model pack is not installed"))?;
        let profile = if profile == "multiview" { "multiview" } else { "full" };
        let installed_index = (!custom_manifest && profile == "full")
            .then(|| home.as_ref().map(|root| root.join("index.sidx")))
            .flatten()
            .filter(|path| path.is_file());
        let index_override =
            env_path_with_fallback("SKWD_LENS_INDEX", "SKWD_SEMANTIC_INDEX").or(installed_index);
        let fixed_index = index_override.is_some();
        let index = index_override.unwrap_or_else(|| {
            let cache = PathBuf::from(cache_dir).join("semantic");
            if !custom_manifest && profile == "full" {
                cache.join("index-siglip2.sidx")
            } else {
                let identity = skwd_lens_proto::manifest_identity(&manifest)
                    .unwrap_or_else(|_| manifest.display().to_string());
                cache.join(skwd_lens_proto::cache_index_name(&identity, profile))
            }
        });
        let root = manifest.parent().unwrap_or_else(|| Path::new("."));
        let runtime = env_path_with_fallback("SKWD_LENS_ORT_DYLIB", "SKWD_SEMANTIC_ORT_DYLIB")
            .or_else(|| home.as_ref().and_then(|path| find_runtime(path)))
            .or_else(|| find_runtime_near(root))
            .or_else(|| installed_manifest(cache_dir).and_then(|path| find_runtime_near(&path)))
            .ok_or_else(|| String::from("semantic ONNX Runtime is not installed"))?;
        let bin = env_path_with_fallback("SKWD_LENS_BIN", "SKWD_SEMANTIC_BIN")
            .or_else(installed_bin)
            .ok_or_else(|| String::from("skwd-lens is not installed"))?;
        let paths = Self { bin, manifest, index, runtime, fixed_index };
        paths.validate()?;
        Ok(paths)
    }

    fn validate(&self) -> Result<(), String> {
        if !executable::is_executable(&self.bin) {
            return Err(format!("Lens helper is not executable at {}", self.bin.display()));
        }
        for (label, path) in
            [("semantic manifest", &self.manifest), ("semantic runtime", &self.runtime)]
        {
            if !path.is_file() {
                return Err(format!("{label} not found at {}", path.display()));
            }
        }
        if self.fixed_index && !self.index.is_file() {
            return Err(format!("semantic index not found at {}", self.index.display()));
        }
        if self.fixed_index && !index_model_matches(self) {
            return Err(format!(
                "semantic index {} was built for a different model pack",
                self.index.display()
            ));
        }
        Ok(())
    }
}

impl SemanticTooling {
    pub fn discover(cache_dir: &str, selected_manifest: &str) -> Result<Self, String> {
        let home = env_path_with_fallback("SKWD_LENS_HOME", "SKWD_SEMANTIC_HOME");
        let selected_root = (!selected_manifest.trim().is_empty())
            .then(|| Path::new(selected_manifest).parent())
            .flatten();
        let data_root = env_path("XDG_DATA_HOME")
            .or_else(|| env_path("HOME").map(|path| path.join(".local/share")))
            .map(|path| path.join("skwd-lens/models/semantic"));
        let runtime = env_path_with_fallback("SKWD_LENS_ORT_DYLIB", "SKWD_SEMANTIC_ORT_DYLIB")
            .or_else(|| home.as_deref().and_then(find_runtime))
            .or_else(|| selected_root.and_then(find_runtime_near))
            .or_else(|| data_root.as_deref().and_then(find_runtime))
            .or_else(|| installed_manifest(cache_dir).and_then(|path| find_runtime_near(&path)))
            .ok_or_else(|| String::from("semantic ONNX Runtime is not installed"))?;
        let bin = env_path_with_fallback("SKWD_LENS_BIN", "SKWD_SEMANTIC_BIN")
            .or_else(installed_bin)
            .ok_or_else(|| String::from("skwd-lens is not installed"))?;
        if !executable::is_executable(&bin) {
            return Err(format!("Lens helper is not executable at {}", bin.display()));
        }
        if !runtime.is_file() {
            return Err(format!("semantic runtime not found at {}", runtime.display()));
        }
        Ok(Self { bin, runtime })
    }
}

impl SemanticService {
    pub fn start(
        paths: SemanticPaths,
        threads: usize,
        deliver: impl Fn(SemanticResult) + Send + 'static,
    ) -> Self {
        let (tx, rx) = mpsc::channel();
        let processes = ProcessSlot::default();
        let worker_processes = processes.clone();
        let stopped = Arc::new(AtomicBool::new(false));
        let worker_stopped = Arc::clone(&stopped);
        std::thread::Builder::new()
            .name(String::from("semantic-search"))
            .spawn(move || {
                service_loop(
                    &paths,
                    threads.max(1),
                    &rx,
                    &deliver,
                    &worker_processes,
                    &worker_stopped,
                );
            })
            .expect("semantic service thread");
        Self { tx, processes, stopped }
    }

    pub fn query(&self, query: SemanticQuery) -> bool {
        self.tx.send(ServiceCommand::Query(query.into())).is_ok()
    }
}

impl Drop for SemanticService {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::Release);
        let _ = self.tx.send(ServiceCommand::Stop);
        self.processes.terminate();
    }
}

impl SemanticChild {
    fn spawn(
        paths: &SemanticPaths,
        threads: usize,
        processes: &ProcessSlot,
        stopped: &AtomicBool,
    ) -> Result<Self, String> {
        let mut child = Command::new(&paths.bin)
            .arg("--serve")
            .arg("--manifest")
            .arg(&paths.manifest)
            .arg("--index")
            .arg(&paths.index)
            .arg("--runtime")
            .arg(&paths.runtime)
            .arg("--threads")
            .arg(threads.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|error| format!("start Lens helper: {error}"))?;
        let active = processes.register(child.id());
        if stopped.load(Ordering::Acquire) {
            let _ = child.kill();
            let _ = child.wait();
            return Err(String::from("semantic search cancelled"));
        }
        let Some(stdin) = child.stdin.take() else {
            let _ = child.kill();
            let _ = child.wait();
            return Err(String::from("semantic stdin unavailable"));
        };
        let Some(stdout) = child.stdout.take() else {
            let _ = child.kill();
            let _ = child.wait();
            return Err(String::from("semantic stdout unavailable"));
        };
        Ok(Self {
            child,
            stdin: BufWriter::new(stdin),
            stdout: BufReader::new(stdout),
            _active: active,
        })
    }

    fn search(&mut self, query: &Query) -> Result<SemanticResult, String> {
        serde_json::to_writer(&mut self.stdin, query)
            .map_err(|error| format!("encode semantic query: {error}"))?;
        self.stdin
            .write_all(b"\n")
            .and_then(|()| self.stdin.flush())
            .map_err(|error| format!("send semantic query: {error}"))?;
        let mut line = String::new();
        let read = self
            .stdout
            .read_line(&mut line)
            .map_err(|error| format!("read semantic response: {error}"))?;
        if read == 0 {
            return Err(String::from("Lens helper exited before responding"));
        }
        let response: Response = serde_json::from_str(&line)
            .map_err(|error| format!("decode semantic response: {error}"))?;
        Ok(SemanticResult {
            generation: response.generation,
            keys: response.matches.into_iter().map(|item| item.key).collect(),
            exclusions: query.exclusions.clone(),
            query_ms: response.query_ms,
            search_ms: response.search_ms,
            error: response.error,
        })
    }
}

impl Drop for SemanticChild {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn service_loop(
    paths: &SemanticPaths,
    threads: usize,
    rx: &Receiver<ServiceCommand>,
    deliver: &impl Fn(SemanticResult),
    processes: &ProcessSlot,
    stopped: &AtomicBool,
) {
    let mut child = None;
    while let Ok(command) = rx.recv() {
        let ServiceCommand::Query(query) = command else { break };
        let Some(query) = debounce_query(rx, query) else { return };
        if stopped.load(Ordering::Acquire) {
            return;
        }
        if child.is_none() {
            if !index_model_matches(paths) {
                deliver(failed(
                    query.generation,
                    String::from("semantic search index is still being prepared by skwd-walld"),
                    query.exclusions,
                ));
                continue;
            }
            match SemanticChild::spawn(paths, threads, processes, stopped) {
                Ok(spawned) => child = Some(spawned),
                Err(error) => {
                    if stopped.load(Ordering::Acquire) {
                        return;
                    }
                    deliver(failed(query.generation, error, query.exclusions));
                    continue;
                }
            }
        }
        let result = child.as_mut().unwrap().search(&query);
        if stopped.load(Ordering::Acquire) {
            return;
        }
        match result {
            Ok(response) => deliver(response),
            Err(error) => {
                child = None;
                deliver(failed(query.generation, error, query.exclusions));
            }
        }
    }
}

fn debounce_query(rx: &Receiver<ServiceCommand>, mut query: Query) -> Option<Query> {
    loop {
        match rx.recv_timeout(query.debounce) {
            Ok(ServiceCommand::Query(next)) => query = next,
            Ok(ServiceCommand::Stop) | Err(RecvTimeoutError::Disconnected) => return None,
            Err(RecvTimeoutError::Timeout) => return Some(query),
        }
    }
}

fn index_model_matches(paths: &SemanticPaths) -> bool {
    let expected = std::fs::read(&paths.manifest)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
        .and_then(|manifest| {
            Some(format!(
                "{}@{}",
                manifest.get("id")?.as_str()?,
                manifest.get("version")?.as_str()?
            ))
        });
    let read_model = || -> Option<String> {
        let mut reader = std::fs::File::open(&paths.index).ok()?;
        let mut magic = [0_u8; 8];
        reader.read_exact(&mut magic).ok()?;
        if &magic != b"SKWDSEM3" {
            return None;
        }
        let mut value = [0_u8; 4];
        reader.read_exact(&mut value).ok()?;
        reader.read_exact(&mut value).ok()?;
        let length = u32::from_le_bytes(value) as usize;
        if length == 0 || length > 4_096 {
            return None;
        }
        let mut model = vec![0_u8; length];
        reader.read_exact(&mut model).ok()?;
        String::from_utf8(model).ok()
    };
    expected.is_some() && expected == read_model()
}

fn failed(generation: u64, error: String, exclusions: Vec<String>) -> SemanticResult {
    SemanticResult {
        generation,
        keys: Vec::new(),
        exclusions,
        query_ms: 0.0,
        search_ms: 0.0,
        error: Some(error),
    }
}

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).filter(|value| !value.is_empty()).map(PathBuf::from)
}

fn env_path_with_fallback(canonical: &str, legacy: &str) -> Option<PathBuf> {
    env_path(canonical).or_else(|| env_path(legacy))
}

fn installed_manifest(cache_dir: &str) -> Option<PathBuf> {
    let executable = std::env::current_exe().ok();
    let data_home = env_path("XDG_DATA_HOME")
        .or_else(|| env_path("HOME").map(|path| path.join(".local/share")));
    installed_manifest_at(Path::new(cache_dir), data_home.as_deref(), executable.as_deref())
}

fn installed_manifest_at(
    cache_dir: &Path,
    data_home: Option<&Path>,
    executable: Option<&Path>,
) -> Option<PathBuf> {
    let parent = executable.and_then(Path::parent);
    let relative = PathBuf::from("semantic-pack.json");
    let candidates = [
        data_home
            .map(|path| path.join("skwd-lens/models/semantic").join(&relative))
            .unwrap_or_default(),
        parent.map(|path| path.join("lens").join(&relative)).unwrap_or_default(),
        parent
            .and_then(Path::parent)
            .map(|path| path.join("share/skwd-lens/models/semantic").join(&relative))
            .unwrap_or_default(),
        cache_dir.join("semantic").join(&relative),
        parent.map(|path| path.join("semantic").join(&relative)).unwrap_or_default(),
        parent
            .and_then(Path::parent)
            .map(|path| path.join("share/skwd-wall/semantic").join(&relative))
            .unwrap_or_default(),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn installed_bin() -> Option<PathBuf> {
    executable::discover(&["skwd-lens", "skwd-wall-semantic"])
}

fn find_runtime(root: &Path) -> Option<PathBuf> {
    let candidates = [
        root.join("runtime/libonnxruntime.so.1.27.0"),
        root.join("runtime/libonnxruntime.so"),
        root.join("runtime/libonnxruntime.dylib"),
        root.join("runtime/onnxruntime.dll"),
        root.join("libonnxruntime.so.1.27.0"),
        root.join("libonnxruntime.so"),
        root.join("libonnxruntime.dylib"),
        root.join("onnxruntime.dll"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn find_runtime_near(root: &Path) -> Option<PathBuf> {
    std::iter::successors(Some(root), |path| path.parent()).take(4).find_map(find_runtime)
}

#[cfg(test)]
mod tests;
