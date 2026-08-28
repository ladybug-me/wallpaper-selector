use std::fs;
use std::path::{Path, PathBuf};

const CALL_NAMES: &[&str] = &["call_tracked", "call"];
const CONFIG_KEY_PATH: &str = "skwd_config::keys::";

#[test]
fn rpc_methods_are_literals() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    rust_files(&root.join("src"), &mut files);
    assert!(!files.is_empty());

    let mut offenders = Vec::new();
    for path in files {
        let relative = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        let source =
            fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {relative}: {error}"));
        for line in config_key_method_lines(&source) {
            offenders.push(format!("{relative}:{line}"));
        }
    }

    assert!(offenders.is_empty(), "config keys used as RPC methods:\n{}", offenders.join("\n"));
}

fn rust_files(directory: &Path, files: &mut Vec<PathBuf>) {
    if !directory.is_dir() {
        return;
    }
    for entry in fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
    {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

fn config_key_method_lines(source: &str) -> Vec<usize> {
    let bytes = source.as_bytes();
    let mut offenders: Vec<usize> = Vec::new();
    let mut line = 1usize;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'\n' {
            line += 1;
            index += 1;
            continue;
        }
        let Some(after_paren) = call_argument_start(bytes, index) else {
            index += 1;
            continue;
        };
        if first_argument(bytes, after_paren).contains(CONFIG_KEY_PATH)
            && offenders.last() != Some(&line)
        {
            offenders.push(line);
        }
        index = after_paren;
    }
    offenders
}

fn call_argument_start(bytes: &[u8], index: usize) -> Option<usize> {
    if index > 0 && is_ident(bytes[index - 1]) {
        return None;
    }
    CALL_NAMES.iter().find_map(|name| {
        let end = index + name.len();
        (bytes.get(index..end) == Some(name.as_bytes()) && bytes.get(end) == Some(&b'('))
            .then_some(end + 1)
    })
}

fn first_argument(bytes: &[u8], start: usize) -> String {
    let mut depth = 0usize;
    let mut index = start;
    while index < bytes.len() {
        match bytes[index] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            b',' if depth == 0 => break,
            b'"' => {
                index += 1;
                while index < bytes.len() && bytes[index] != b'"' {
                    index += usize::from(bytes[index] == b'\\') + 1;
                }
            }
            _ => {}
        }
        index += 1;
    }
    String::from_utf8_lossy(&bytes[start..index.min(bytes.len())]).into_owned()
}

fn is_ident(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphanumeric()
}

#[test]
fn scanner_flags_config_keys() {
    let wrapped = concat!(
        "app.call_tracked(\n",
        "    skwd_config::keys::wallhaven::COLLECTIONS,\n",
        "    serde_json::json!({}),\n",
        "    Pending::BrowserCollections,\n",
        ");\n",
    );
    assert_eq!(config_key_method_lines(wrapped), [1]);
    let inline = "self.daemon.client.call(skwd_config::keys::theme::BACKEND, json!({}));\n";
    assert_eq!(config_key_method_lines(inline), [1]);
}

#[test]
fn scanner_allows_literals() {
    let allowed = concat!(
        "app.call_tracked(\"wallhaven.collections\", json!({}), Pending::BrowserCollections);\n",
        "self.daemon.client.call(\"wall.retheme\", json!({}));\n",
        "app.config.save_key(skwd_config::keys::theme::BACKEND, json!(value));\n",
        "app.call_tracked(\"config.set\", json!({\"k\": skwd_config::keys::theme::BACKEND}), p);\n",
        "let value = self.recall(skwd_config::keys::theme::BACKEND);\n",
    );
    assert!(config_key_method_lines(allowed).is_empty());
}
