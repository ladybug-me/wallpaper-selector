use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

pub fn find(
    names: &[&str],
    sibling_directory: Option<&Path>,
    search_path: Option<&OsStr>,
) -> Option<PathBuf> {
    for name in names {
        if let Some(directory) = sibling_directory {
            let candidate = directory.join(name);
            if is_executable(&candidate) {
                return Some(candidate);
            }
        }
        if let Some(value) = search_path {
            for directory in std::env::split_paths(value) {
                let candidate = directory.join(name);
                if is_executable(&candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

pub fn discover(names: &[&str]) -> Option<PathBuf> {
    let executable = std::env::current_exe().ok();
    let sibling_directory = executable.as_deref().and_then(Path::parent);
    find(names, sibling_directory, std::env::var_os("PATH").as_deref())
}

#[cfg(test)]
mod tests;
