mod commands;
mod state;
mod window;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_single_instance::init(|app, _args, _cwd| {
                // 二次启动：聚焦已有主窗口
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.unminimize();
                    let _ = w.set_focus();
                }
            }),
        )
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            state::setup_state(app)?;
            window::setup_tray(app.handle())?;
            if let Some(w) = app.get_webview_window("main") {
                window::hide_on_close(&w);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::profile::open_nas])
        .run(tauri::generate_context!())
        .expect("error while running fnos-desktop");
}
