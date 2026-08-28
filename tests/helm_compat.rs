#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[test]
fn helm_exec_passes_argv() {
    let directory = tempfile::tempdir().unwrap();
    let helm = directory.path().join("skwd-helm");
    std::fs::write(&helm, b"#!/bin/sh\nprintf '%s\\n' \"$@\"\nexit 42\n").unwrap();
    std::fs::set_permissions(&helm, std::fs::Permissions::from_mode(0o700)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_skwd-wall"))
        .env("SKWD_HELM_BIN", &helm)
        .args(["apply", "wallpaper.png", "--output", "DP-1"])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(42));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "apply\nwallpaper.png\n--output\nDP-1\n");
}
