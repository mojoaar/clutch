mod api;
mod cancel;
mod context;
mod db;
mod export;
mod file_resolver;
mod logs;
mod model_cache;

mod sessions;
mod settings;
mod skills;
mod user_profile;
mod web_fetcher;
mod workspaces;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{prelude::*, reload, EnvFilter, Registry};
use std::sync::OnceLock;

type FilterHandle = reload::Handle<EnvFilter, Registry>;
static RELOAD_HANDLE: OnceLock<FilterHandle> = OnceLock::new();

#[cfg(target_os = "macos")]
const SHORTCUT_MAIN: &str = "CmdOrCtrl+Ctrl+KeyK";

#[cfg(not(target_os = "macos"))]
const SHORTCUT_MAIN: &str = "CmdOrCtrl+Alt+KeyK";


#[tauri::command]
fn toggle_dev_mode(window: tauri::WebviewWindow, enabled: bool) {
    if enabled {
        let _ = window.open_devtools();
        if let Some(handle) = RELOAD_HANDLE.get() {
            let _ = handle.modify(|filter| {
                *filter = EnvFilter::new("debug");
            });
        }
    } else {
        let _ = window.close_devtools();
        if let Some(handle) = RELOAD_HANDLE.get() {
            let _ = handle.modify(|filter| {
                *filter = EnvFilter::new("info");
            });
        }
    }
}

/// Application entry point. Initializes logging, database, plugins, tray icon, global shortcuts,
/// and window lifecycle management (start minimized, close to tray).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from("."));
    let log_dir = std::path::PathBuf::from(home).join(".clutch").join("logs");
    std::fs::create_dir_all(&log_dir).ok();

    let file_appender = tracing_appender::rolling::daily(&log_dir, "clutch.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = if cfg!(debug_assertions) {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    let (filter_layer, reload_handle) = reload::Layer::new(filter);
    RELOAD_HANDLE.set(reload_handle).ok();

    if cfg!(debug_assertions) {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::CLOSE))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter_layer)
            .with(tracing_subscriber::fmt::layer().with_writer(non_blocking).json())
            .init();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None::<Vec<&str>>))
        .invoke_handler(tauri::generate_handler![
            api::stream_chat,
            api::abort_stream,
            model_cache::get_models,
            model_cache::refresh_models,
            model_cache::get_cached_models,
            context::count_message_tokens,
            context::get_context_limit,
            context::auto_trim_context,

            sessions::list_sessions,
            sessions::create_session,
            sessions::update_session_title,
            sessions::update_session_provider,
            sessions::pin_session,
            sessions::unpin_session,
            sessions::archive_session,
            sessions::unarchive_session,
            sessions::delete_session,
            sessions::list_messages,
            sessions::create_message,
            sessions::update_message,
            sessions::delete_message,
            settings::get_setting,
            settings::set_setting,
            settings::get_all_settings,
            settings::get_app_version,
            settings::test_connection,
            settings::get_balance,
            user_profile::get_user_profile,
            user_profile::update_user_profile,
            workspaces::list_workspaces,
            workspaces::add_workspace,
            workspaces::remove_workspace,
            workspaces::set_active_workspace,
            workspaces::get_active_workspace,
            workspaces::detect_workspaces,
            workspaces::read_workspace_file,
            workspaces::write_workspace_file,
            workspaces::list_workspace_dir,
            workspaces::create_workspace_dir,
            workspaces::delete_workspace_dir,
            skills::get_skill_detail,
            skills::install_skill,
            skills::uninstall_skill,
            skills::list_installed_skills,
            skills::get_skill_instructions,
            skills::execute_skill_action,
            skills::check_skill_updates,
            skills::update_skill,
            web_fetcher::fetch_url,
            web_fetcher::batch_fetch,
            web_fetcher::fetch_github_readme,
            web_fetcher::fetch_webpage_info,
            file_resolver::resolve_and_read_file,
            export::export_session,
            logs::get_logs,
            settings::toggle_tray_icon,
            toggle_dev_mode,
        ])
        .setup(|app| {
            let db_path = app.path().app_data_dir()?.join("clutch.db");

            match tauri::async_runtime::block_on(db::init_pool(db_path)) {
                Ok(pool) => {
                    app.manage(pool);
                    app.manage(cancel::StreamCancelState::new());
                }
                Err(e) => {
                    tracing::error!("Failed to initialize database: {}", e);
                    return Err(e.to_string().into());
                }
            }

            {
                let pool = app.state::<sqlx::SqlitePool>().inner().clone();
                if let Ok(Some(val)) = tauri::async_runtime::block_on(
                    sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = 'dev_mode'")
                        .fetch_optional(&pool),
                ) {
                    if val == "true" {
                        if let Some(handle) = RELOAD_HANDLE.get() {
                            let _ = handle.modify(|filter| {
                                *filter = EnvFilter::new("debug");
                            });
                        }
                    }
                }
            }

            let shortcut_main = {
                let shortcut_pool = app.state::<sqlx::SqlitePool>().inner().clone();
                tauri::async_runtime::block_on(
                    sqlx::query_scalar::<_, String>(
                        "SELECT value FROM settings WHERE key = 'shortcut_main'"
                    )
                    .fetch_optional(&shortcut_pool),
                )
                .ok()
                .flatten()
                .unwrap_or_else(|| SHORTCUT_MAIN.to_string())
            };

            let new_chat = MenuItemBuilder::with_id("new_chat", "New Chat").build(app)?;
            let show_app = MenuItemBuilder::with_id("show", "Show App").build(app)?;
            let show_settings = MenuItemBuilder::with_id("settings", "Show Settings").build(app)?;
            let show_shortcuts = MenuItemBuilder::with_id("shortcuts", "Show Shortcuts").build(app)?;
            let exit = MenuItemBuilder::with_id("exit", "Exit App").build(app)?;

            let shortcut_label = show_app.clone();

            let gs = app.global_shortcut();

            let _ = gs.unregister_all();

            if let Err(e) = gs.on_shortcut(shortcut_main.as_str(), move |app, _shortcut, event| {
                if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    if let Some(window) = app.get_webview_window("main") {
                        let visible = window.is_visible().unwrap_or(false);
                        if visible {
                            let _ = window.hide();
                            let _ = shortcut_label.set_text("Show App");
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = shortcut_label.set_text("Hide App");
                        }
                    }
                }
            }) {
                tracing::error!("Failed to attach main handler: {:?}", e);
            }

            tracing::debug!("Shortcut registered: main={}",
                gs.is_registered(shortcut_main.as_str()));

            let tray_menu = MenuBuilder::new(app)
                .item(&new_chat)
                .item(&show_app)
                .item(&show_settings)
                .item(&show_shortcuts)
                .separator()
                .item(&exit)
                .build()?;

            let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../icons/tray-icon.png"))
                .expect("Failed to load tray icon");

            let tray_show1 = show_app.clone();
            let tray_show2 = show_app.clone();

            let tray = TrayIconBuilder::with_id("main-tray")
                .icon(tray_icon)
                .icon_as_template(true)
                .tooltip("Clutch")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    let id = event.id().as_ref();
                    match id {
                "new_chat" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.eval("window.location.href = '/new'");
                        let _ = tray_show1.set_text("Hide App");
                    }
                }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                            let _ = tray_show1.set_text("Show App");
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = tray_show1.set_text("Hide App");
                        }
                    }
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.eval("window.location.href = '/settings'");
                        let _ = tray_show1.set_text("Hide App");
                    }
                }
                "shortcuts" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.eval("window.location.href = '/settings?tab=shortcuts'");
                        let _ = tray_show1.set_text("Hide App");
                    }
                }
                        "exit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(move |tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = tray_show2.set_text("Hide App");
                        }
                    }
                })
                .build(app)?;

            app.manage(tray);

            // Read show_tray_icon setting — hide tray immediately if disabled
            let pool = app.state::<sqlx::SqlitePool>().inner().clone();
            let show_tray = tauri::async_runtime::block_on(
                sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = 'show_tray_icon'")
                    .fetch_optional(&pool)
            ).ok().flatten().unwrap_or_else(|| "true".to_string()) == "true";

            if !show_tray {
                if let Some(tray) = app.tray_by_id("main-tray") {
                    let _ = tray.set_visible(false);
                }
            }

            // Startup behavior: read both settings from DB
            // Window is created with visible:false — show it unless minimized

            let start_minimized = tauri::async_runtime::block_on(
                sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = 'start_minimized'")
                    .fetch_optional(&pool)
            ).ok().flatten().unwrap_or_default();

            if let Some(window) = app.get_webview_window("main") {
                // Show the window if not minimized
                if start_minimized != "true" {
                    let _ = window.show();
                }

                // Close to tray: intercept CloseRequested, hide instead of quitting
                let window_clone = window.clone();
                let pool2 = pool.clone();
                let tray_show3 = show_app.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        let should_tray = tauri::async_runtime::block_on(
                            sqlx::query_scalar::<_, String>(
                                "SELECT value FROM settings WHERE key = 'close_to_tray'"
                            ).fetch_optional(&pool2)
                        ).ok().flatten().unwrap_or_default() == "true";

                        if should_tray {
                            api.prevent_close();
                            let _ = window_clone.hide();
                            let _ = tray_show3.set_text("Show App");
                        }
                    }
                });
            }

            tracing::info!("Clutch initialized. Tray icon and global shortcuts registered.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
