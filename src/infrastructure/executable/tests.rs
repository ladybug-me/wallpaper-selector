use super::*;
use std::os::unix::fs::PermissionsExt;

fn executable(path: &Path) {
    std::fs::write(path, b"#!/bin/sh\n").unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
}

#[test]
fn name_order_beats_sibling() {
    let root = tempfile::tempdir().unwrap();
    let sibling = root.path().join("sibling");
    let path = root.path().join("path");
    std::fs::create_dir_all(&sibling).unwrap();
    std::fs::create_dir_all(&path).unwrap();
    executable(&sibling.join("legacy"));
    executable(&path.join("canonical"));

    assert_eq!(
        find(&["canonical", "legacy"], Some(&sibling), Some(path.as_os_str())),
        Some(path.join("canonical"))
    );
}

#[test]
fn sibling_beats_path() {
    let root = tempfile::tempdir().unwrap();
    let sibling = root.path().join("sibling");
    let path = root.path().join("path");
    std::fs::create_dir_all(&sibling).unwrap();
    std::fs::create_dir_all(&path).unwrap();
    executable(&sibling.join("canonical"));
    executable(&path.join("canonical"));

    assert_eq!(
        find(&["canonical"], Some(&sibling), Some(path.as_os_str())),
        Some(sibling.join("canonical"))
    );
}

#[test]
fn non_executable_skipped() {
    let root = tempfile::tempdir().unwrap();
    let sibling = root.path().join("sibling");
    let path = root.path().join("path");
    std::fs::create_dir_all(&sibling).unwrap();
    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(sibling.join("canonical"), b"not executable").unwrap();
    executable(&path.join("canonical"));

    assert_eq!(
        find(&["canonical"], Some(&sibling), Some(path.as_os_str())),
        Some(path.join("canonical"))
    );
}
