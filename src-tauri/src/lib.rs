use tauri::{http::Response, Manager};
use tokio::sync::Mutex;

use crate::{app_service::AppService, utils::parse_pages_uri};

pub mod app_service;
mod commands;
pub mod library;
pub mod metadata;
pub mod metafile;
pub mod reader;
pub mod settings;
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
        .register_uri_scheme_protocol("pages", |app, request| {
            match parse_pages_uri(request.uri().path()) {
                Ok((id, file_num, page_num)) => {
                    let state = app.app_handle().state::<Mutex<AppService>>();
                    let content = tauri::async_runtime::block_on(async {
                        state.lock().await.get_page(&id, file_num, page_num).await
                    });

                    match content {
                        Ok(page) => Response::builder().status(200).body(page).unwrap(),
                        Err(e) => {
                            log::error!(
                                "Could not get page {} for id {} and file {}",
                                page_num,
                                id,
                                file_num
                            );
                            Response::builder()
                                .status(400)
                                .body(e.to_string().into_bytes())
                                .unwrap()
                        }
                    }
                }
                Err(e) => {
                    log::error!("{e}");
                    Response::builder()
                        .status(400)
                        .body(e.into_bytes())
                        .unwrap()
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::download,
            commands::search,
            commands::list_torrents,
            commands::toggle_pause,
            commands::list_library,
            commands::delete,
            commands::remove_download,
            commands::load_cbz,
            commands::update_reading_progress,
            commands::update_library_entry_settings,
            commands::get_dimensions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
