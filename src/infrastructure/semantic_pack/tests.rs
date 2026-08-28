use std::os::unix::fs::PermissionsExt;

use super::*;

#[test]
fn helper_report_decodes() {
    let directory = tempfile::tempdir().unwrap();
    let helper = directory.path().join("skwd-lens");
    std::fs::write(
        &helper,
        br#"#!/bin/sh
printf '%s\n' '{"format":1,"id":"model/id","version":"v1","dimensions":768,"manifest":"/models/pack/semantic-pack.json","bytes":42}'
"#,
    )
    .unwrap();
    std::fs::set_permissions(&helper, std::fs::Permissions::from_mode(0o700)).unwrap();
    let report = import_source(
        &helper,
        Path::new("/runtime.so"),
        Path::new("/source.skwdmodel"),
        directory.path(),
    )
    .unwrap();
    assert_eq!(report.id, "model/id");
    assert_eq!(report.dimensions, 768);
    assert_eq!(report.bytes, 42);
}

#[test]
fn helper_failure_surfaces() {
    let directory = tempfile::tempdir().unwrap();
    let helper = directory.path().join("skwd-lens");
    std::fs::write(&helper, b"#!/bin/sh\nprintf 'bad pack' >&2\nexit 2\n").unwrap();
    std::fs::set_permissions(&helper, std::fs::Permissions::from_mode(0o700)).unwrap();
    let error = import_source(
        &helper,
        Path::new("/runtime.so"),
        Path::new("/source.skwdmodel"),
        directory.path(),
    )
    .unwrap_err();
    assert_eq!(error, "bad pack");
}
