use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_channel::mpsc::UnboundedSender;
use log::{debug, info};
use serde_json::Value;


use crate::infrastructure::runtime::Wake;

#[derive(Debug, Clone)]
pub enum IpcMsg {
    Connected { list_id: u64 },
    Disconnected,
    Response { id: u64, result: Option<Value>, error: Option<wall_proto::ErrorInfo> },
    Event { name: String, data: Value },
}

#[derive(Clone)]
pub struct IpcHandle {
    next_id: Arc<AtomicU64>,
    tx: Arc<Mutex<Option<UnboundedSender<Wake>>>>,
}

impl IpcHandle {
    #[cfg(test)]
    pub(super) fn disconnected() -> Self {
        Self { next_id: Arc::new(AtomicU64::new(1)), tx: Arc::new(Mutex::new(None)) }
    }

    fn get_wallpapers_json() -> serde_json::Value {
        let mut wallpapers = Vec::new();
        if let Some(mut path) = dirs::picture_dir() {
            path.push("Wallpapers");
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            let name = entry.file_name().to_string_lossy().into_owned();
                            let abs_path = entry.path().to_string_lossy().into_owned();
                            let lower = name.to_lowercase();
                            if lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg") || lower.ends_with(".webp") || lower.ends_with(".gif") {
                                wallpapers.push(serde_json::json!({
                                    "name": name,
                                    "key": abs_path,
                                    "path": abs_path,
                                    "type": "static",
                                    "thumb": abs_path,
                                }));
                            }
                        }
                    }
                }
            }
        }
        serde_json::json!({ "schema_version": 1, "wallpapers": wallpapers })
    }

    #[cfg_attr(test, allow(dead_code))]
    pub fn start(tx: UnboundedSender<Wake>) -> Self {
        let handle = Self { next_id: Arc::new(AtomicU64::new(1)), tx: Arc::new(Mutex::new(Some(tx.clone()))) };
        
        let list_id = handle.next_id.fetch_add(1, Ordering::Relaxed);
        let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Connected { list_id }));
        
        let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Response {
            id: list_id,
            result: Some(Self::get_wallpapers_json()),
            error: None,
        }));
        
        let watcher_tx = tx.clone();
        std::thread::Builder::new()
            .name("ipc-reader".into())
            .spawn(move || watcher_loop(&watcher_tx))
            .expect("spawn ipc reader");
            
        handle
    }

    pub fn call(&self, method: &str, params: Value) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        
        let tx_guard = self.tx.lock().unwrap();
        let Some(tx) = tx_guard.as_ref() else { return id; };

        match method {
            "wall.apply" => {
                let path = params.get("path").and_then(Value::as_str).unwrap_or("");
                if !path.is_empty() {
                    info!("Caelestia setting wallpaper: {}", path);
                    let _ = std::process::Command::new("caelestia")
                        .args(["wallpaper", "-f", path])
                        .spawn();
                    
                    let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Event {
                        name: wall_proto::ev::APPLIED.to_string(),
                        data: serde_json::json!({ "key": path }),
                    }));
                }
                
                let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Response {
                    id, result: Some(serde_json::json!({})), error: None,
                }));
            }
            "theme.update" => {
                info!("Caelestia setting scheme via UI");
                let _ = std::process::Command::new("caelestia")
                    .args(["scheme", "set", "--name", "dynamic"]) // Simplest fallback
                    .spawn();
                let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Response {
                    id, result: Some(serde_json::json!({})), error: None,
                }));
            }
            "theme.get" => {
                let mut palette = serde_json::json!({});
                if let Ok(output) = std::process::Command::new("caelestia").args(["scheme", "get"]).output() {
                    let out_str = String::from_utf8_lossy(&output.stdout);
                    // Minimal parsing to make the UI happy if it expects colors
                    for line in out_str.lines() {
                        let parts: Vec<&str> = line.trim().splitn(2, ':').collect();
                        if parts.len() == 2 {
                            palette[parts[0].trim()] = serde_json::Value::String(parts[1].trim().to_string());
                        }
                    }
                }
                let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Response {
                    id, result: Some(serde_json::json!({ "palette": palette })), error: None,
                }));
            }
            "subscribe" | "wall.outputs" | "task.status" | "wall.list" | "effects.list" 
            | "theme.backends" | "playlist.list" | "playlist.update" | "playlist.create" 
            | "playlist.assign" | "playlist.delete" | "wall.set_audio" | "wall.update_tags" 
            | "optimize.start" | "wall.shell_preview" | "wall.shell_preview_end" | "wall.retheme" | "status" => {
                let result = if method == "wall.outputs" {
                    serde_json::json!({ "schema_version": 1, "outputs": [{"name": "*", "id": "*"}] })
                } else if method == "wall.list" {
                    Self::get_wallpapers_json()
                } else if method == "effects.list" {
                    serde_json::json!({ "schema_version": 1, "effects": [] })
                } else if method.ends_with(".list") || method == "theme.backends" {
                    serde_json::json!({ "schema_version": 1, "items": [] })
                } else if method == "theme.preview" {
                    serde_json::json!({ "schema_version": 1, "colors": [] })
                } else if method == "status" {
                    serde_json::json!({ "version": env!("CARGO_PKG_VERSION") })
                } else {
                    serde_json::json!({})
                };
                
                let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Response {
                    id, result: Some(result), error: None,
                }));
            }
            _ => {
                debug!("unsupported IPC method in Caelestia mode: {}", method);
                let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Response {
                    id, result: Some(serde_json::json!({})), error: None,
                }));
            }
        }
        
        id
    }
}

fn watcher_loop(tx: &UnboundedSender<Wake>) {
    let mut last_mtime = std::time::SystemTime::UNIX_EPOCH;
    let state_home = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".local/state")
        });
    let state_path = state_home.join("caelestia/wallpaper/path.txt");

    loop {
        if let Ok(meta) = std::fs::metadata(&state_path) {
            if let Ok(mtime) = meta.modified() {
                if mtime > last_mtime && last_mtime != std::time::SystemTime::UNIX_EPOCH {
                    if let Ok(content) = std::fs::read_to_string(&state_path) {
                        let _ = tx.unbounded_send(Wake::Ipc(IpcMsg::Event {
                            name: wall_proto::ev::APPLIED.to_string(),
                            data: serde_json::json!({ "key": content.trim() }),
                        }));
                    }
                }
                last_mtime = mtime;
            }
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}

#[cfg(test)]
mod tests;
