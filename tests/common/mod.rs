use std::fs;
use std::path::{Path, PathBuf};

pub fn production_rust_files(directory: &Path, files: &mut Vec<PathBuf>) {
    if !directory.is_dir() {
        return;
    }
    for entry in fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
    {
        let entry = entry.expect("directory entry");
        let path = entry.path();
        let name = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if path.is_dir() {
            if name != "tests" && name != "target" {
                production_rust_files(&path, files);
            }
        } else if path.extension().is_some_and(|extension| extension == "rs")
            && name != "tests.rs"
            && !name.ends_with("_tests.rs")
            && !is_cfg_test_file(&path)
        {
            files.push(path);
        }
    }
}

fn is_cfg_test_file(path: &Path) -> bool {
    let source =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    source.lines().next().is_some_and(|line| line.trim() == "#![cfg(test)]")
}
