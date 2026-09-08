use std::sync::Mutex;
use tauri::Manager;

use crate::commands::profile::ServerProfile;

/// 应用全局状态。
/// M1：内存占位；M2 引入 profiles.json 持久化（tauri-plugin-store）。
#[derive(Default)]
pub struct AppState {
    pub profiles: Mutex<Vec<ServerProfile>>,
}

pub fn setup_state<R: tauri::Runtime>(app: &tauri::App<R>) -> tauri::Result<()> {
    app.manage(AppState::default());
    Ok(())
}
