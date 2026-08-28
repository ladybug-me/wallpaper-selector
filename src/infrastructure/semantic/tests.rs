use super::*;

#[test]
fn wire_query_keys() {
    let wire = serde_json::to_value(Query::from(SemanticQuery {
        generation: 4,
        text: String::from("rain"),
        negative_query: Some(String::from("people")),
        exclusions: vec![String::from("nsfw")],
        top_k: 12,
        max_results: 256,
        debounce: Duration::ZERO,
    }))
    .unwrap();
    let mut keys: Vec<&str> = wire.as_object().unwrap().keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "generation",
            "maxResults",
            "minResults",
            "minScoreProminence",
            "negativeQuery",
            "negativeWeight",
            "query",
            "scoreWindow",
            "topK",
        ]
    );
    assert_eq!(wire["minResults"], serde_json::json!(0));
    assert_eq!(wire["scoreWindow"], serde_json::json!(SCORE_WINDOW));
    assert_eq!(wire["minScoreProminence"], serde_json::json!(MIN_SCORE_PROMINENCE));
    assert_eq!(wire["negativeWeight"], serde_json::json!(NEGATIVE_WEIGHT));
    assert_eq!(wire["query"], serde_json::json!("rain"));
    assert_eq!(wire["maxResults"], serde_json::json!(256));
    assert_eq!(wire["topK"], serde_json::json!(12));
}

#[test]
fn failed_result_keeps_generation() {
    let result = failed(19, String::from("no model"), vec![String::from("people")]);
    assert_eq!(result.generation, 19);
    assert_eq!(result.error.as_deref(), Some("no model"));
    assert!(result.keys.is_empty());
    assert_eq!(result.exclusions, ["people"]);
}

#[test]
fn runtime_lookup_accepts_packaged_layout() {
    let directory = tempfile::tempdir().unwrap();
    let runtime = directory.path().join("runtime");
    std::fs::create_dir(&runtime).unwrap();
    let library = runtime.join("libonnxruntime.so.1.27.0");
    std::fs::write(&library, []).unwrap();
    assert_eq!(find_runtime(directory.path()), Some(library));
}

#[test]
fn default_product_pack() {
    let directory = tempfile::tempdir().unwrap();
    let semantic = directory.path().join("semantic");
    std::fs::create_dir_all(&semantic).unwrap();
    let obsolete_tier = semantic.join("packs/performance");
    std::fs::create_dir_all(&obsolete_tier).unwrap();
    std::fs::write(obsolete_tier.join("semantic-pack.json"), b"{}").unwrap();
    assert_eq!(installed_manifest_at(directory.path(), None, None), None);

    let manifest = semantic.join("semantic-pack.json");
    std::fs::write(&manifest, b"{}").unwrap();
    assert_eq!(installed_manifest_at(directory.path(), None, None), Some(manifest));
}

#[test]
fn canonical_pack_precedence() {
    let directory = tempfile::tempdir().unwrap();
    let canonical = directory.path().join("data/skwd-lens/models/semantic/semantic-pack.json");
    let legacy = directory.path().join("cache/semantic/semantic-pack.json");
    std::fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    std::fs::create_dir_all(legacy.parent().unwrap()).unwrap();
    std::fs::write(&canonical, b"{}").unwrap();
    std::fs::write(&legacy, b"{}").unwrap();

    assert_eq!(
        installed_manifest_at(
            &directory.path().join("cache"),
            Some(&directory.path().join("data")),
            None,
        ),
        Some(canonical)
    );
}

#[test]
fn packaged_prefix_discovery() {
    let directory = tempfile::tempdir().unwrap();
    let executable = directory.path().join("usr/bin/skwd-wall");
    let semantic = directory.path().join("usr/share/skwd-lens/models/semantic");
    let manifest = semantic.join("semantic-pack.json");
    let runtime = semantic.join("runtime/libonnxruntime.so.1.27.0");
    std::fs::create_dir_all(executable.parent().unwrap()).unwrap();
    std::fs::create_dir_all(runtime.parent().unwrap()).unwrap();
    std::fs::write(&executable, []).unwrap();
    std::fs::write(&manifest, b"{}").unwrap();
    std::fs::write(&runtime, []).unwrap();

    assert_eq!(
        installed_manifest_at(&directory.path().join("cache"), None, Some(&executable)),
        Some(manifest.clone())
    );
    assert_eq!(find_runtime_near(manifest.parent().unwrap()), Some(runtime));
}

#[cfg(unix)]
#[test]
fn helper_shutdown_sigkill() {
    assert_eq!(HELPER_SHUTDOWN_SIGNAL, libc::SIGKILL);
    assert_ne!(HELPER_SHUTDOWN_SIGNAL, libc::SIGTERM);
}

#[test]
fn fixed_index_model_mismatch() {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("semantic-pack.json");
    std::fs::write(&manifest, br#"{"id":"model","version":"two"}"#).unwrap();
    let index = directory.path().join("index.sidx");
    let mut bytes = Vec::from(b"SKWDSEM3".as_slice());
    bytes.extend_from_slice(&512_u32.to_le_bytes());
    bytes.extend_from_slice(&9_u32.to_le_bytes());
    bytes.extend_from_slice(b"model@one");
    std::fs::write(&index, bytes).unwrap();
    let paths = SemanticPaths {
        bin: PathBuf::new(),
        manifest,
        index,
        runtime: PathBuf::new(),
        fixed_index: true,
    };

    assert!(!index_model_matches(&paths));
    assert!(paths.validate().is_err());
}

#[cfg(unix)]
#[test]
fn daemon_index_queryable() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempfile::tempdir().unwrap();
    let helper = directory.path().join("semantic-helper");
    std::fs::write(
        &helper,
        br#"#!/bin/sh
while IFS= read -r line; do
    printf '%s\n' '{"generation":7,"queryMs":1.0,"searchMs":1.0,"matches":[],"error":null}'
done
"#,
    )
    .unwrap();
    std::fs::set_permissions(&helper, std::fs::Permissions::from_mode(0o700)).unwrap();
    let manifest = directory.path().join("semantic-pack.json");
    std::fs::write(&manifest, br#"{"id":"model","version":"one"}"#).unwrap();
    let index = directory.path().join("index.sidx");
    let mut bytes = Vec::from(b"SKWDSEM3".as_slice());
    bytes.extend_from_slice(&512_u32.to_le_bytes());
    bytes.extend_from_slice(&9_u32.to_le_bytes());
    bytes.extend_from_slice(b"model@one");
    std::fs::write(&index, bytes).unwrap();
    let runtime = directory.path().join("libonnxruntime.so");
    std::fs::write(&runtime, []).unwrap();
    let paths = SemanticPaths { bin: helper, manifest, index, runtime, fixed_index: false };
    let (tx, rx) = mpsc::channel();
    let service = SemanticService::start(paths, 1, move |result| {
        tx.send(result).unwrap();
    });
    assert!(service.query(SemanticQuery {
        generation: 7,
        text: String::from("rain"),
        negative_query: None,
        exclusions: Vec::new(),
        top_k: 1,
        max_results: 1,
        debounce: super::QUERY_DEBOUNCE,
    }));

    let result = rx
        .recv_timeout(Duration::from_millis(1_000))
        .expect("daemon-owned index should be queryable");
    assert_eq!(result.generation, 7);
    assert!(result.error.is_none());
}

#[cfg(unix)]
#[test]
fn drop_terminates_helper() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempfile::tempdir().unwrap();
    let helper = directory.path().join("semantic-helper");
    std::fs::write(&helper, b"#!/bin/sh\nprintf '%s' \"$$\" > \"$0.pid\"\nexec sleep 30\n")
        .unwrap();
    std::fs::set_permissions(&helper, std::fs::Permissions::from_mode(0o700)).unwrap();
    let pid_path = helper.with_extension("pid");
    let manifest = directory.path().join("semantic-pack.json");
    std::fs::write(&manifest, br#"{"id":"model","version":"one"}"#).unwrap();
    let index = directory.path().join("index.sidx");
    let mut bytes = Vec::from(b"SKWDSEM3".as_slice());
    bytes.extend_from_slice(&512_u32.to_le_bytes());
    bytes.extend_from_slice(&9_u32.to_le_bytes());
    bytes.extend_from_slice(b"model@one");
    std::fs::write(&index, bytes).unwrap();
    let runtime = directory.path().join("libonnxruntime.so");
    std::fs::write(&runtime, []).unwrap();
    let paths = SemanticPaths { bin: helper, manifest, index, runtime, fixed_index: true };
    let service = SemanticService::start(paths, 1, |_| {});
    assert!(service.query(SemanticQuery {
        generation: 1,
        text: String::from("rain"),
        negative_query: None,
        exclusions: Vec::new(),
        top_k: 1,
        max_results: 1,
        debounce: super::QUERY_DEBOUNCE,
    }));

    let pid = (0..200)
        .find_map(|_| {
            let pid =
                std::fs::read_to_string(&pid_path).ok().and_then(|value| value.parse::<u32>().ok());
            if pid.is_none() {
                std::thread::sleep(Duration::from_millis(10));
            }
            pid
        })
        .expect("helper should publish a complete PID");
    drop(service);

    let process_path = PathBuf::from(format!("/proc/{pid}"));
    for _ in 0..200 {
        if !process_path.exists() {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(!process_path.exists(), "helper {pid} alive");
}

#[test]
fn typing_burst_last_query() {
    let (tx, rx) = mpsc::channel();
    let sender = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(5));
        tx.send(ServiceCommand::Query(Query {
            generation: 2,
            text: String::from("cozy bedroom"),
            negative_query: None,
            negative_weight: 0.5,
            top_k: 10,
            score_window: 0.035,
            min_score_prominence: 0.015,
            max_results: 256,
            min_results: 8,
            exclusions: Vec::new(),
            debounce: Duration::from_millis(25),
        }))
        .unwrap();
        std::thread::sleep(Duration::from_millis(5));
        tx.send(ServiceCommand::Query(Query {
            generation: 3,
            text: String::from("cozy bedroom rainstorm"),
            negative_query: None,
            negative_weight: 0.5,
            top_k: 10,
            score_window: 0.035,
            min_score_prominence: 0.015,
            max_results: 256,
            min_results: 8,
            exclusions: Vec::new(),
            debounce: Duration::from_millis(25),
        }))
        .unwrap();
        std::thread::sleep(Duration::from_millis(40));
    });
    let first = Query {
        generation: 1,
        text: String::from("cozy"),
        negative_query: None,
        negative_weight: 0.5,
        top_k: 10,
        score_window: 0.035,
        min_score_prominence: 0.015,
        max_results: 256,
        min_results: 8,
        exclusions: Vec::new(),
        debounce: Duration::from_millis(25),
    };

    let query = debounce_query(&rx, first).unwrap();

    sender.join().unwrap();
    assert_eq!(query.generation, 3);
    assert_eq!(query.text, "cozy bedroom rainstorm");
}

#[test]
fn zero_debounce_immediate() {
    let (_tx, rx) = mpsc::channel();
    let query = Query {
        generation: 9,
        text: String::from("red haired anime woman"),
        negative_query: None,
        negative_weight: 0.5,
        top_k: 10,
        score_window: 0.035,
        min_score_prominence: 0.015,
        max_results: 256,
        min_results: 0,
        exclusions: Vec::new(),
        debounce: Duration::ZERO,
    };
    let started = std::time::Instant::now();
    assert_eq!(debounce_query(&rx, query).unwrap().generation, 9);
    assert!(started.elapsed() < Duration::from_millis(50));
}
