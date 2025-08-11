use tauri::Manager;
use tokio::sync::Mutex;

use crate::app_service::AppService;

pub mod app_service;
mod commands;
pub mod metadata;
pub mod source;
pub mod torrent;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        // logging for rqbit is quite verbose so im turning it off here...
                        .level_for("librqbit_dht", log::LevelFilter::Off)
                        .level_for("librqbit_tracker_comms", log::LevelFilter::Off)
                        .level_for("librqbit", log::LevelFilter::Off)
                        .level_for("tracing", log::LevelFilter::Off)
                        .build(),
                )?;
            }
            let app_service = tauri::async_runtime::block_on(async {
                let app_dir = app.path().app_data_dir().expect("data dir is missing");
                AppService::new(app_dir).await
            })
            .expect("Failed to create app service");
            app.manage(Mutex::new(app_service));
            log::info!("Setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::download,
            commands::search,
            commands::list_torrents,
            commands::toggle_pause
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
