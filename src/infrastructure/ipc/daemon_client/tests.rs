#![cfg(test)]

use super::*;

#[test]
fn socket_transport_drains_empty() {
    let client = DaemonClient::disconnected();
    assert_eq!(client.call("wall.list", serde_json::json!({})), 0);
    assert!(client.drain_calls().is_empty());
}

#[test]
fn recording_transport_drains_once() {
    let client = DaemonClient::recording();
    let first = client.call("wall.apply", serde_json::json!({"path": "a.png"}));
    let second = client.call("wall.list", serde_json::json!({}));
    assert!(second > first);
    let calls = client.drain_calls();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].0, "wall.apply");
    assert_eq!(calls[1].0, "wall.list");
    assert!(client.drain_calls().is_empty());
}
