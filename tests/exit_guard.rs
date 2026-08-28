use std::fs;
use std::path::Path;

#[test]
fn exit_via_hard_exit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = vec![root.join("src/main.rs")];
    for entry in fs::read_dir(root.join("src/app")).expect("read src/app") {
        let path = entry.expect("dir entry").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    let mut offenders = Vec::new();
    for path in files {
        let rel = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        let src = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {rel}: {err}"));
        for (index, line) in src.lines().enumerate() {
            if line.contains("process::exit(0)") {
                offenders.push(format!("{rel}:{}", index + 1));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "process::exit(0) segfaults the NVIDIA Vulkan driver during teardown; use \
         crate::hard_exit(0): {offenders:?}"
    );
}
