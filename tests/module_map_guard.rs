mod common;
use common::production_rust_files;

use std::fs;
use std::path::Path;

fn declared_module(line: &str) -> Option<&str> {
    let mut line = line.trim_start();
    if let Some(after_pub) = line.strip_prefix("pub") {
        if let Some(scoped) = after_pub.strip_prefix('(') {
            line = scoped.split_once(')')?.1.trim_start();
        } else if let Some(spaced) = after_pub.strip_prefix(char::is_whitespace) {
            line = spaced.trim_start();
        }
    }
    let declaration = line.strip_prefix("mod ")?.trim_end();
    declaration
        .strip_suffix(';')
        .or_else(|| declaration.strip_suffix("{}"))
        .or_else(|| declaration.strip_suffix('{'))
        .map(str::trim)
}

fn inline_test_marker(line: &str) -> Option<&'static str> {
    let line = line.trim_start();
    let inline_module = line
        .strip_prefix("mod tests")
        .or_else(|| line.strip_prefix("pub mod tests"))
        .or_else(|| line.strip_prefix("pub(crate) mod tests"))
        .is_some_and(|remainder| remainder.trim_start().starts_with('{'));
    if inline_module {
        return Some("inline `mod tests`");
    }
    let attribute = line.strip_prefix("#[")?.split(['(', ']']).next()?.trim();
    if attribute == "test"
        || attribute.ends_with("::test")
        || matches!(attribute, "rstest" | "test_case" | "wasm_bindgen_test")
    {
        Some("inline test function")
    } else {
        None
    }
}

#[test]
fn compound_modules_use_directory_maps() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    production_rust_files(&root.join("src"), &mut files);
    production_rust_files(&root.join("crates"), &mut files);

    let mut offenders = Vec::new();
    for source_path in files {
        let file_name =
            source_path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if matches!(file_name, "main.rs" | "lib.rs" | "mod.rs") {
            continue;
        }
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", source_path.display()));
        for (line_index, line) in source.lines().enumerate() {
            let Some(module) = declared_module(line) else {
                continue;
            };
            if module.contains("test") {
                continue;
            }
            let relative = source_path.strip_prefix(root).unwrap_or(&source_path);
            offenders.push(format!(
                "{}:{} declares `{module}`",
                relative.display(),
                line_index + 1
            ));
        }
    }

    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn unit_tests_in_siblings() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    production_rust_files(&root.join("src"), &mut files);
    production_rust_files(&root.join("crates"), &mut files);

    let mut offenders = Vec::new();
    for source_path in files {
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", source_path.display()));
        for (line_index, line) in source.lines().enumerate() {
            let Some(marker) = inline_test_marker(line) else {
                continue;
            };
            let relative = source_path.strip_prefix(root).unwrap_or(&source_path);
            offenders.push(format!("{}:{} contains {marker}", relative.display(), line_index + 1));
        }
    }

    assert!(offenders.is_empty(), "{offenders:?}");
}
