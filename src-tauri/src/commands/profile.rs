use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use tauri::AppHandle;

use crate::window;

/// 服务器 Profile（M1 定义数据模型，M2 接入持久化 CRUD）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerProfile {
    pub id: String,
    pub name: String,
    pub url: String,
    pub trust_cert: bool,
    pub zoom: f64,
    pub created_at: i64,
    pub last_used_at: i64,
}

/// M1：按 URL 打开 NAS 窗口（独立 WebView 分区）。
/// M2 起改为：list_profiles / add_profile / open_profile。
#[tauri::command]
pub fn open_nas(app: AppHandle, url: String) -> Result<String, String> {
    let url = url.trim().to_string();
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("地址需以 http:// 或 https:// 开头（如 https://192.168.1.16:5667）".into());
    }
    let label = nas_label(&url);
    window::open_nas_window(&app, &label, &url)?;
    Ok(label)
}

/// 由 URL 生成稳定窗口标签：同一地址复用同一窗口。
pub fn nas_label(url: &str) -> String {
    let mut h = DefaultHasher::new();
    url.hash(&mut h);
    format!("nas_{:016x}", h.finish())
}
