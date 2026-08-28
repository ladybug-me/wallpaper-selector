mod common;
use common::production_rust_files;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const DOMAIN_FORBIDDEN: &[&str] = &[
    "crate::app",
    "crate::backend",
    "crate::contracts",
    "crate::data",
    "crate::frontend",
    "crate::infrastructure",
    "crate::rendering",
    "iced::",
    "libc::",
    "reqwest::",
    "rusqlite::",
    "serde_json::",
    "skwd_config::",
    "std::env",
    "std::fs",
    "std::net",
    "std::process",
    "std::thread",
    "tokio::",
    "ureq::",
    "wall_proto::",
    "wayland_",
    "wgpu::",
];

const BACKEND_FORBIDDEN: &[&str] = &[
    "crate::app",
    "crate::data",
    "crate::frontend",
    "crate::infrastructure",
    "crate::rendering",
    "iced::",
    "iced_wgpu::",
    "libc::",
    "reqwest::",
    "rusqlite::",
    "serde_json::",
    "skwd_config::",
    "std::env",
    "std::fs",
    "std::net",
    "std::process",
    "std::thread",
    "tokio::",
    "ureq::",
    "wall_proto::",
    "wayland_",
    "wgpu::",
];

const FRONTEND_FORBIDDEN: &[&str] = &[
    "crate::backend",
    "crate::data",
    "crate::infrastructure",
    "crate::rendering",
    "iced_wgpu::",
    "libc::",
    "reqwest::",
    "rusqlite::",
    "serde_json::",
    "skwd_config::",
    "std::env",
    "std::fs",
    "std::net",
    "std::process",
    "std::thread",
    "tokio::",
    "ureq::",
    "wayland_",
    "wall_proto::",
    "wgpu::",
];

const REUSABLE_UI_FORBIDDEN: &[&str] = &["crate::app"];

const BROWSER_FRONTEND_FORBIDDEN: &[&str] = &[
    "crate::backend",
    "crate::infrastructure",
    "crate::rendering",
    "Config",
    "libc::",
    "reqwest::",
    "rusqlite::",
    "serde_json",
    "skwd_config::",
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::process",
    "std::thread",
    "tokio::",
    "ureq::",
    "wall_proto::",
    "wayland_",
    "wgpu::",
];

const BROWSER_CONTRACTS_FORBIDDEN: &[&str] = &[
    "crate::app",
    "crate::backend",
    "crate::frontend",
    "crate::infrastructure",
    "crate::rendering",
    "Config",
    "iced::",
    "iced_runtime::",
    "libc::",
    "reqwest::",
    "rusqlite::",
    "serde_json",
    "skwd_config::",
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::process",
    "std::thread",
    "tokio::",
    "ureq::",
    "wall_proto::",
    "wayland_",
    "wgpu::",
];

const INFRASTRUCTURE_FORBIDDEN: &[&str] = &[
    "crate::app",
    "crate::backend",
    "crate::data",
    "crate::frontend",
    "crate::rendering",
    "iced::",
    "iced_wgpu::",
    "wgpu::",
];

const CONTRACTS_FORBIDDEN: &[&str] = &[
    "crate::app",
    "crate::backend",
    "crate::data",
    "crate::frontend",
    "crate::infrastructure",
    "crate::rendering",
    "iced::",
    "iced_wgpu::",
    "std::fs",
    "std::net",
    "std::process",
    "std::thread",
    "wgpu::",
];

const RENDERING_FORBIDDEN: &[&str] = &[
    "crate::app",
    "crate::backend",
    "crate::data",
    "crate::frontend",
    "crate::infrastructure",
    "crate::shell",
    "libc::",
    "reqwest::",
    "serde_json::",
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::path",
    "std::process",
    "std::thread",
    "tokio::",
    "ureq::",
    "wall_proto::",
    "wayland_",
];

fn dependency_offenders(layer: &str, forbidden_dependencies: &[&str]) -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    production_rust_files(&root.join("src").join(layer), &mut files);

    let mut offenders = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        for forbidden in forbidden_dependencies {
            if source.contains(forbidden) {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                offenders.push(format!("{} imports {forbidden}", relative.display()));
            }
        }
    }
    offenders
}

#[test]
fn domain_layer_deps() {
    let offenders = dependency_offenders("domain", DOMAIN_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn backend_layer_deps() {
    let offenders = dependency_offenders("backend", BACKEND_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn frontend_layer_deps() {
    let offenders = dependency_offenders("frontend", FRONTEND_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn reusable_ui_does_not_depend_on_app_messages() {
    let offenders = dependency_offenders("frontend/ui", REUSABLE_UI_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn browser_frontend_deps() {
    let offenders = dependency_offenders("frontend/browser", BROWSER_FRONTEND_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn browser_contract_deps() {
    let offenders = dependency_offenders("contracts/browser", BROWSER_CONTRACTS_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn legacy_browser_gone() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("src/browser").exists());

    for module in [
        "src/frontend/browser/mod.rs",
        "src/contracts/browser/mod.rs",
        "src/infrastructure/browser/mod.rs",
    ] {
        let source = fs::read_to_string(root.join(module)).expect("read browser module map");
        for implementation in ["struct Browser", "enum Source", "fn encode_search", "fn view("] {
            assert!(!source.contains(implementation), "{module}: {implementation}");
        }
    }
}

#[test]
fn infrastructure_layer_deps() {
    let offenders = dependency_offenders("infrastructure", INFRASTRUCTURE_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn contract_globals() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    production_rust_files(&root.join("src").join("contracts"), &mut files);

    let sanctioned = [
        ("src/contracts/display.rs", "SURFACE_SCALE"),
        ("src/contracts/preview/encoding.rs", "COMPRESSED_THUMBNAILS"),
    ];
    let mut offenders = Vec::new();
    for path in files {
        let relative = path.strip_prefix(root).unwrap_or(&path).display().to_string();
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        for (index, line) in source.lines().enumerate() {
            let line = line.trim_start();
            let declaration = line
                .strip_prefix("pub static ")
                .or_else(|| line.strip_prefix("pub(crate) static "))
                .or_else(|| line.strip_prefix("static "));
            let Some(declaration) = declaration else { continue };
            let name = declaration.split([':', ' ']).next().unwrap_or_default();
            if !sanctioned.iter().any(|(file, global)| relative == *file && name == *global) {
                offenders.push(format!("{relative}:{} declares `{name}`", index + 1));
            }
        }
    }
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn contract_layer_deps() {
    let offenders = dependency_offenders("contracts", CONTRACTS_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn rendering_layer_deps() {
    let offenders = dependency_offenders("rendering", RENDERING_FORBIDDEN);
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn renderer_contract_owners() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let snapshot = fs::read_to_string(root.join("src/contracts/rendering/snapshot.rs"))
        .expect("read renderer snapshot contract");
    for owner in ["struct RendererSnapshot", "struct InstanceRaw", "struct TransSnap"] {
        assert!(snapshot.contains(owner), "missing neutral {owner}");
    }

    let preview = fs::read_to_string(root.join("src/contracts/rendering/preview.rs"))
        .expect("read preview port contract");
    assert!(preview.contains("trait PreviewRenderer"));
    assert!(!preview.contains("iced::"));
    assert!(!root.join("src/frontend/effects/preview.rs").exists());

    let frontend_snapshot = fs::read_to_string(root.join("src/frontend/scene/snapshot.rs"))
        .expect("read presentation snapshot");
    for renderer_owner in ["struct RendererSnapshot", "struct InstanceRaw", "struct TransSnap"] {
        assert!(!frontend_snapshot.contains(renderer_owner), "frontend owns {renderer_owner}");
    }
}

#[test]
fn rendering_pipeline_map() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("src/rendering/scene/pipeline.rs").exists());

    let pipeline_map = fs::read_to_string(root.join("src/rendering/scene/pipeline/mod.rs"))
        .expect("read rendering pipeline module map");
    for implementation in ["struct ", "enum ", "trait ", "impl ", "fn "] {
        assert!(
            !pipeline_map.lines().any(|line| line.trim_start().starts_with(implementation)),
            "{implementation}"
        );
    }

    let gpu_probe = fs::read_to_string(root.join("src/rendering/capabilities/gpu.rs"))
        .expect("read rendering GPU probe");
    for adapter_detail in ["std::env", "std::fs", "crate::shell", "PathBuf"] {
        assert!(!gpu_probe.contains(adapter_detail), "{adapter_detail}");
    }
    assert!(root.join("src/infrastructure/capabilities/gpu_cache.rs").exists());
}

#[test]
fn effects_layer_maps() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("src/effects").exists());

    let mut files = Vec::new();
    production_rust_files(&root.join("src"), &mut files);
    let legacy_references: Vec<_> = files
        .into_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
            source
                .contains("crate::effects")
                .then(|| path.strip_prefix(root).unwrap_or(&path).display().to_string())
        })
        .collect();
    assert!(legacy_references.is_empty(), "{legacy_references:?}");

    for module in [
        "src/domain/effects/mod.rs",
        "src/frontend/effects/mod.rs",
        "src/infrastructure/effects/mod.rs",
        "src/rendering/effects/mod.rs",
    ] {
        let source = fs::read_to_string(root.join(module)).expect("read effects module map");
        for implementation in ["struct Effects", "fn decode_definitions", "impl Pipeline"] {
            assert!(!source.contains(implementation), "{module}: {implementation}");
        }
    }
}

#[test]
fn legacy_ui_facade_gone() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("src/ui").exists());

    let mut files = Vec::new();
    production_rust_files(&root.join("src"), &mut files);
    let offenders: Vec<_> = files
        .into_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
            source
                .contains("crate::ui")
                .then(|| path.strip_prefix(root).unwrap_or(&path).display().to_string())
        })
        .collect();
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn legacy_roots_gone() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let removed = [
        "audio_panel",
        "browser",
        "components",
        "data",
        "effects",
        "gpu",
        "metrics",
        "nav",
        "playlists",
        "scene",
        "schedule_editor",
        "settings_ui",
        "tagcloud",
        "theme_designer",
        "timing",
        "ui",
    ];
    for module in removed {
        assert!(!root.join("src").join(module).exists(), "src/{module}");
    }

    let mut files = Vec::new();
    production_rust_files(&root.join("src"), &mut files);
    let forbidden: Vec<String> =
        removed.into_iter().map(|module| format!("crate::{module}")).collect();
    let mut offenders = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for facade in &forbidden {
            if source.contains(facade) {
                let relative = path.strip_prefix(root).unwrap_or(&path);
                offenders.push(format!("{} imports {facade}", relative.display()));
            }
        }
    }
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn module_roots_thin() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut pending = vec![root.join("src")];
    let crates = root.join("crates");
    if crates.is_dir() {
        pending.push(crates);
    }
    let mut offenders = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        {
            let path = entry.expect("module-map entry").path();
            if path.is_dir() {
                if path.file_name().is_none_or(|name| name != "tests") {
                    pending.push(path);
                }
                continue;
            }
            let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
            if name != "mod.rs" && name != "lib.rs" {
                continue;
            }
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            for (line_index, line) in source.lines().enumerate() {
                let line = line.trim_start();
                let implementation = [
                    "fn ",
                    "pub fn ",
                    "pub(crate) fn ",
                    "struct ",
                    "pub struct ",
                    "enum ",
                    "pub enum ",
                    "trait ",
                    "pub trait ",
                    "impl ",
                ]
                .iter()
                .find(|prefix| line.starts_with(**prefix));
                if let Some(prefix) = implementation {
                    let relative = path.strip_prefix(root).unwrap_or(&path);
                    offenders.push(format!(
                        "{}:{} starts implementation with `{prefix}`",
                        relative.display(),
                        line_index + 1
                    ));
                }
            }
        }
    }
    assert!(offenders.is_empty(), "{offenders:?}");
}

#[test]
fn update_dispatcher_thin() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let module_map =
        fs::read_to_string(root.join("src/app/update/mod.rs")).expect("read app update module map");
    let dispatcher = fs::read_to_string(root.join("src/app/update/dispatcher.rs"))
        .expect("read app update dispatcher");

    for module in [
        "browser",
        "effects",
        "filters",
        "lifecycle",
        "navigation",
        "panels",
        "picker",
        "playlists",
        "schedule",
        "settings",
        "tags",
        "theme",
        "ui_command",
    ] {
        assert!(module_map.contains(&format!("mod {module};")), "mod {module}");
    }

    for implementation in [
        "fn mouse_moved(",
        "fn open_browser(",
        "fn pin_to_workspace(",
        "fn settings_input(",
        "fn tick(",
    ] {
        assert!(!dispatcher.contains(implementation), "{implementation}");
    }
}

#[test]
fn daemon_result_handlers_are_typed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let results = root.join("src/app/runtime/results");
    let mut offenders = Vec::new();
    for entry in fs::read_dir(&results).expect("read result handlers") {
        let path = entry.expect("result handler entry").path();
        if path.extension().is_none_or(|extension| extension != "rs")
            || path.file_name().is_some_and(|name| name == "dispatcher.rs")
        {
            continue;
        }
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for raw in ["serde_json::Value", "Value::", "&Value", "serde_json::from_value"] {
            if source.contains(raw) {
                offenders.push(format!("{} contains {raw}", path.display()));
            }
        }
    }
    assert!(offenders.is_empty(), "{offenders:?}");

    let dispatcher =
        fs::read_to_string(results.join("dispatcher.rs")).expect("read daemon result dispatcher");
    assert_eq!(dispatcher.matches("serde_json::Value").count(), 1);
    assert_eq!(dispatcher.matches("&Value").count(), 1);
    assert_eq!(dispatcher.matches("result: &Value").count(), 1);
    for inspection in
        ["result.get(", "Value::", "result.as_array(", "result.as_str(", "serde_json::from_value"]
    {
        assert!(!dispatcher.contains(inspection), "{inspection}");
    }
    assert!(dispatcher.contains("self.rpc_error(request_for_error.clone(), message)"));
    assert!(dispatcher.contains("decode_library_list(result, paths)"));
    assert!(!dispatcher.contains("T::default()"));

    for handler in ["src/app/runtime/library.rs", "src/app/update/scene_properties.rs"] {
        let source = fs::read_to_string(root.join(handler)).expect("read typed result handler");
        assert!(!source.contains("serde_json::Value"), "{handler}");
        assert!(!source.contains("&Value"), "{handler}");
    }

    let playlists = fs::read_to_string(root.join("src/frontend/playlists/model.rs"))
        .expect("read playlist presentation model");
    assert!(!playlists.contains("wall_proto::"));

    let contracts = fs::read_to_string(root.join("src/contracts/daemon.rs"))
        .expect("read consumer-owned daemon contracts");
    assert!(!contracts.contains("wall_proto::"));
    assert!(!contracts.contains("serde_json::"));

    for owner in [
        "src/app/state/daemon.rs",
        "src/app/state/tasks.rs",
        "src/frontend/settings/folio/folio.rs",
        "src/frontend/settings/tabs/builder.rs",
        "src/frontend/ui/bar/action.rs",
        "src/frontend/ui/bar/model/builder.rs",
        "src/frontend/ui/bar/model/types.rs",
        "src/frontend/ui/bar/view.rs",
    ] {
        let source = fs::read_to_string(root.join(owner)).expect("read daemon consumer");
        for wire_type in [
            "wall_proto::OutputStatus",
            "wall_proto::LibraryWatchStatus",
            "wall_proto::LibraryWatchRootStatus",
            "wall_proto::TaskStatus",
            "wall_proto::TaskState",
            "wall_proto::TaskCapabilities",
            "wall_proto::TaskControl",
        ] {
            assert!(!source.contains(wire_type), "{owner} contains {wire_type}");
        }
    }
}

#[test]
fn accessor_owners() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let accessors = root.join("src/infrastructure/config/picker/accessors");
    assert!(!root.join("src/infrastructure/config/picker/accessors.rs").exists());

    let module_map =
        fs::read_to_string(accessors.join("mod.rs")).expect("read picker accessor module map");
    let declared: BTreeSet<String> = module_map
        .lines()
        .filter_map(|line| line.trim().strip_prefix("mod ")?.strip_suffix(';').map(str::to_owned))
        .collect();
    let owners: BTreeSet<String> = fs::read_dir(&accessors)
        .expect("read picker accessor directory")
        .map(|entry| entry.expect("picker accessor entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .filter_map(|path| {
            let stem = path.file_stem()?.to_str()?.to_owned();
            (stem != "mod").then_some(stem)
        })
        .collect();
    assert!(!owners.is_empty());
    assert_eq!(declared, owners);
}
