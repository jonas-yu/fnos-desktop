use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{
    AppHandle, Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent,
};

/// 创建/聚焦 NAS 窗口（M1 使用默认 WebView 分区）。
///
/// 注意：Tauri 2 在 Windows 上 `data_directory` + 动态外部 URL 窗口存在空白问题，
/// M1 先去掉独立分区保证能加载；M2 重新实现多 NAS 登录态隔离。
/// NAS 窗口关闭即销毁（登录态由 WebView 持久化，重开自动恢复）。
pub fn open_nas_window<R: Runtime>(
    app: &AppHandle<R>,
    label: &str,
    url: &str,
) -> Result<(), String> {
    if let Some(w) = app.get_webview_window(label) {
        let _ = w.show();
        let _ = w.set_focus();
        return Ok(());
    }

    let parsed = url
        .parse::<tauri::Url>()
        .map_err(|e| format!("无效地址: {e}"))?;

    WebviewWindowBuilder::new(app, label, WebviewUrl::External(parsed))
        .title("飞牛 NAS")
        .inner_size(1200.0, 800.0)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 主窗口：关窗隐藏到托盘（真正退出走托盘菜单）
pub fn hide_on_close(win: &WebviewWindow) {
    let w = win.clone();
    win.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = w.hide();
        }
    });
}

/// 系统托盘：显示主界面 / 锁定（M3）/ 退出
pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let show_i = MenuItem::with_id(app, "show", "显示主界面", true, None::<&str>)?;
    let lock_i = MenuItem::with_id(app, "lock", "锁定", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &lock_i, &quit_i])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main(app),
            "quit" => app.exit(0),
            "lock" => {
                // M3：锁屏
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        builder = builder.icon(icon);
    }

    builder.build(app)?;
    Ok(())
}

fn show_main<R: Runtime>(app: &AppHandle<R>) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
    }
}
