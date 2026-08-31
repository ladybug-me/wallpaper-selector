use std::path::PathBuf;

const VERBS: &[&str] = &["apply", "next", "prev", "random", "status", "list", "theme"];

pub fn try_delegate(arguments: &[String]) -> Option<i32> {
    if !is_helm_command(arguments) {
        return None;
    }
    
    // Map helm verbs to Caelestia CLI commands where possible
    let mut mapped_args = Vec::new();
    let verb = arguments.first().map(|s| s.as_str()).unwrap_or("");
    
    match verb {
        "apply" | "set" => {
            mapped_args.push("wallpaper".to_string());
            mapped_args.push("-f".to_string());
            if let Some(path) = arguments.get(1) {
                mapped_args.push(path.clone());
            }
        }
        "next" | "prev" | "random" => {
            mapped_args.push("wallpaper".to_string());
            mapped_args.push("-r".to_string());
        }
        _ => {
            // Default to passing to Caelestia wallpaper
            mapped_args.push("wallpaper".to_string());
            mapped_args.extend(arguments.iter().skip(1).cloned());
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;

        let error = std::process::Command::new("caelestia").args(&mapped_args).exec();
        eprintln!("skwd-wall: cannot delegate to caelestia: {error}");
        Some(3)
    }
    #[cfg(not(unix))]
    {
        match std::process::Command::new("caelestia").args(&mapped_args).status() {
            Ok(status) => Some(status.code().unwrap_or(1)),
            Err(error) => {
                eprintln!("skwd-wall: cannot delegate to caelestia: {error}");
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

#[path = "helm_compat_tests.rs"]
mod tests;
