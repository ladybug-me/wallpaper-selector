#[cfg(all(test, unix))]
mod tests;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct PickerLabels {
    pub title: String,
    pub packs: String,
    pub all_files: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportedSemanticPack {
    pub format: u32,
    pub id: String,
    pub version: String,
    pub dimensions: usize,
    pub manifest: PathBuf,
    pub bytes: u64,
}

pub fn choose_and_import(
    bin: &Path,
    runtime: &Path,
    labels: &PickerLabels,
) -> Result<Option<ImportedSemanticPack>, String> {
    let Some(source) = choose_source(labels)? else {
        return Ok(None);
    };
    import_source(bin, runtime, &source, &models_directory()?).map(Some)
}

pub fn import_source(
    bin: &Path,
    runtime: &Path,
    source: &Path,
    models_dir: &Path,
) -> Result<ImportedSemanticPack, String> {
    let output = Command::new(bin)
        .arg("--install-pack")
        .arg(source)
        .arg("--models-dir")
        .arg(models_dir)
        .arg("--runtime")
        .arg(runtime)
        .output()
        .map_err(|error| format!("start Lens model-pack installer: {error}"))?;
    let output = checked(output, "install model pack")?;
    serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("read Lens install report: {error}"))
}

fn choose_source(labels: &PickerLabels) -> Result<Option<PathBuf>, String> {
    let mut arguments = vec![
        String::from("--file-selection"),
        format!("--title={}", labels.title),
        format!(
            "--file-filter={} | semantic-pack.json *.skwdmodel *.zip *.tar *.tar.gz *.tgz *.tar.xz *.txz *.tar.zst *.tzst",
            labels.packs
        ),
        format!("--file-filter={} | *", labels.all_files),
    ];
    if let Some(downloads) = home_directory().map(|home| home.join("Downloads"))
        && downloads.is_dir()
    {
        arguments.push(format!("--filename={}/", downloads.display()));
    }
    let output = Command::new("zenity").args(&arguments).output().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            String::from("zenity is required for the model-pack file picker")
        } else {
            error.to_string()
        }
    })?;
    if output.status.success() {
        let selected = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        return Ok((!selected.is_empty()).then(|| PathBuf::from(selected)));
    }
    if output.status.code() == Some(1) {
        return Ok(None);
    }
    Err(output_error(&output, "model-pack file picker"))
}

fn models_directory() -> Result<PathBuf, String> {
    let data = env::var_os("XDG_DATA_HOME")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .or_else(|| home_directory().map(|home| home.join(".local/share")))
        .ok_or_else(|| String::from("cannot locate the user data directory"))?;
    Ok(data.join("skwd-lens/models/semantic/packs"))
}

fn home_directory() -> Option<PathBuf> {
    env::var_os("HOME").filter(|path| !path.is_empty()).map(PathBuf::from)
}

fn checked(output: Output, operation: &str) -> Result<Output, String> {
    if output.status.success() {
        return Ok(output);
    }
    Err(output_error(&output, operation))
}

fn output_error(output: &Output, operation: &str) -> String {
    let detail = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if detail.is_empty() { format!("cannot {operation}") } else { detail }
}
