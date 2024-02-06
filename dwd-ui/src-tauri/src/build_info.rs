use serde::Serialize;
use shadow_rs::shadow;
use specta::Type;

#[derive(Type, Serialize)]
pub struct BuildInfos {
    rust_version: String,
    build_time: String,
    build_os: String,
}

#[tauri::command]
pub fn build_infos() -> BuildInfos {
    shadow!(build);
    let rust_version = build::RUST_VERSION.into();
    let build_time = build::BUILD_TIME.into();
    let build_os = build::BUILD_OS.into();
    BuildInfos {
        rust_version,
        build_time,
        build_os,
    }
}
