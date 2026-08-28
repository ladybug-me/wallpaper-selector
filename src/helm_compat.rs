use std::path::PathBuf;

use wall_proto::helm::VERBS;

pub fn try_delegate(arguments: &[String]) -> Option<i32> {
    if !is_helm_command(arguments) {
        return None;
    }
    let binary = helm_binary();
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;

        let error = std::process::Command::new(&binary).args(arguments).exec();
        eprintln!("skwd-wall: cannot delegate to {}: {error}", binary.display());
        Some(3)
    }
    #[cfg(not(unix))]
    {
        match std::process::Command::new(&binary).args(arguments).status() {
            Ok(status) => Some(status.code().unwrap_or(1)),
            Err(error) => {
                eprintln!("skwd-wall: cannot delegate to {}: {error}", binary.display());
                Some(3)
            }
        }
    }
}

fn is_helm_command(arguments: &[String]) -> bool {
    arguments.first().is_some_and(|argument| {
        argument == "--help" || argument == "-h" || VERBS.contains(&argument.as_str())
    })
}

fn helm_binary() -> PathBuf {
    if let Some(binary) = std::env::var_os("SKWD_HELM_BIN") {
        return PathBuf::from(binary);
    }
    if let Ok(executable) = std::env::current_exe()
        && let Some(directory) = executable.parent()
    {
        let sibling = directory.join("skwd-helm");
        if sibling.is_file() {
            return sibling;
        }
    }
    PathBuf::from("skwd-helm")
}

#[path = "helm_compat_tests.rs"]
mod tests;
