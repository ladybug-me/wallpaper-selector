#![cfg(test)]

use super::*;

fn test_dir(label: &str) -> PathBuf {
    static NEXT_DIR: AtomicU64 = AtomicU64::new(1);
    loop {
        let dir = std::env::temp_dir().join(format!(
            "skwd-test-{label}-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        match std::fs::create_dir(&dir) {
            Ok(()) => return dir,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => panic!("create test directory {}: {error}", dir.display()),
        }
    }
}

#[test]
fn wait_for_socket_binds() {
    let dir = test_dir("ipc");
    std::fs::create_dir_all(&dir).unwrap();
    let sock = dir.join("wall.sock");
    let sock2 = sock.clone();
    let server = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(50));
        let listener = std::os::unix::net::UnixListener::bind(&sock2).unwrap();
        let _ = listener.accept();
    });
    let start = std::time::Instant::now();
    assert!(wait_for_socket(&sock, Duration::from_secs(2)));
    assert!(start.elapsed() < Duration::from_millis(500));
    let _ = server.join();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn preview_ready_forwarded() {
    let path = "/tmp/remote-preview-original.png";
    let (tx, mut rx) = futures_channel::mpsc::unbounded();
    let line = serde_json::json!({
        "event": wall_proto::ev::PREVIEW_READY,
        "data": {"id": "remote-1", "path": path},
    })
    .to_string();
    dispatch_line(&line, &tx);
    let IpcMsg::Event { name, data } = next_msg(&mut rx, "preview ready") else {
        panic!("expected prepared preview event");
    };
    assert_eq!(name, wall_proto::ev::PREVIEW_READY);
    assert_eq!(data["id"], "remote-1");
    assert_eq!(data["path"], path);
}

#[test]
fn wait_for_socket_timeout() {
    assert!(!wait_for_socket(
        std::path::Path::new("/nonexistent-skwd/wall.sock"),
        Duration::from_millis(30)
    ));
}

#[test]
fn spawn_throttle() {
    assert!(may_spawn_walld(false, None));
    assert!(!may_spawn_walld(true, None));
    assert!(!may_spawn_walld(true, Some(Duration::from_secs(120))));
    assert!(!may_spawn_walld(false, Some(Duration::from_secs(29))));
    assert!(may_spawn_walld(false, Some(Duration::from_secs(30))));
}

fn next_msg(rx: &mut futures_channel::mpsc::UnboundedReceiver<Wake>, stage: &str) -> IpcMsg {
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        match rx.try_recv() {
            Ok(Wake::Ipc(msg)) => return msg,
            Ok(_) => {}
            Err(futures_channel::mpsc::TryRecvError::Closed) => {
                panic!("wake channel closed")
            }
            Err(_) => {
                assert!(std::time::Instant::now() < deadline, "timed out at {stage}");
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }
}

#[test]
fn reader_loop_dispatch() {
    let dir = test_dir("reader");
    std::fs::create_dir_all(&dir).unwrap();
    let sock = dir.join("wall.sock");
    let listener = std::os::unix::net::UnixListener::bind(&sock).unwrap();
    let (tx, mut rx) = futures_channel::mpsc::unbounded();
    let handle = IpcHandle::disconnected();
    let reader_handle = handle.clone();
    let sock2 = sock.clone();
    std::thread::spawn(move || reader_loop(&reader_handle, &tx, Some(sock2.as_path())));

    let (conn, _) = listener.accept().unwrap();
    let mut reader = BufReader::new(conn.try_clone().unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    assert!(line.starts_with(r#"{"method":"subscribe","params":"#), "outbound frame = {line}");
    let sub: wall_proto::Request =
        serde_json::from_str(&line).expect("outbound frame decodes as wall_proto::Request");
    assert_eq!(sub.method, "subscribe");
    assert_eq!(sub.params["events"], serde_json::json!(["skwd."]));
    line.clear();
    reader.read_line(&mut line).unwrap();
    let list: wall_proto::Request =
        serde_json::from_str(&line).expect("outbound frame decodes as wall_proto::Request");
    assert_eq!(list.method, "wall.list");
    assert_eq!(list.params["favourites"], serde_json::json!(false));
    let wire_list_id = list.id;

    let IpcMsg::Connected { list_id } = next_msg(&mut rx, "connected") else {
        panic!("first message must be Connected");
    };
    assert_eq!(list_id, wire_list_id);

    let mut writer = conn.try_clone().unwrap();
    writeln!(writer, r#"{{"id":{wire_list_id},"result":{{"wallpapers":[]}}}}"#).unwrap();
    writeln!(writer, r#"{{"event":"skwd.wall.applied","data":{{"name":"a.png"}}}}"#).unwrap();
    writeln!(writer, "this is not json").unwrap();
    writeln!(writer, r#"{{"id":7,"error":{{"code":-1,"message":"nope"}}}}"#).unwrap();

    match next_msg(&mut rx, "list response") {
        IpcMsg::Response { id, result, error } => {
            assert_eq!(id, wire_list_id);
            assert!(error.is_none());
            assert_eq!(result.unwrap()["wallpapers"], serde_json::json!([]));
        }
        other => panic!("expected Response, got {other:?}"),
    }
    match next_msg(&mut rx, "event") {
        IpcMsg::Event { name, data } => {
            assert_eq!(name, "skwd.wall.applied");
            assert_eq!(data["name"], "a.png");
        }
        other => panic!("expected Event, got {other:?}"),
    }
    match next_msg(&mut rx, "error response") {
        IpcMsg::Response { id, result, error } => {
            assert_eq!(id, 7);
            assert!(result.is_none());
            let error = error.unwrap();
            assert_eq!(error.message, "nope");
            assert_eq!(error.code, -1);
        }
        other => panic!("expected error Response, got {other:?}"),
    }
    std::fs::remove_file(&sock).unwrap();
    conn.shutdown(std::net::Shutdown::Both).unwrap();
    assert!(matches!(next_msg(&mut rx, "disconnected"), IpcMsg::Disconnected));
    assert_eq!(handle.call("wall.list", serde_json::json!({})), 0, "0 = dropped");
    drop(listener);
    let _ = std::fs::remove_dir_all(&dir);
}
