use std::time::{Duration, Instant};

use tauri::{Emitter, State};
use tokio::sync::Mutex;

use crate::app_service::{AppService, SearchResponse};

#[tauri::command]
pub async fn download(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppService>>,
    id: String,
) -> Result<(), String> {
    app_handle
        .emit("download-started", &id)
        .map_err(|e| e.to_string())?;

    state
        .lock()
        .await
        .download(&id)
        .await
        .map_err(|e| e.to_string())?;

    let mut rx = state
        .lock()
        .await
        .get_torrent_stats_receiver(&id)
        .await
        .map_err(|e| e.to_string())?;

    tokio::spawn({
        let app_handle = app_handle.clone();
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                if rx.changed().await.is_err() {
                    break;
                }
                let stats = rx.borrow();
                app_handle
                    .emit("download-progress", stats.clone())
                    .map_err(|e| e.to_string())?;
            }

            Ok::<(), String>(())
        }
    });

    state
        .lock()
        .await
        .torrent_service
        .lock()
        .await
        .wait_until_finished(&id)
        .await
        .map_err(|e| e.to_string())?;

    app_handle
        .emit("download-completed", &id)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn search(
    state: State<'_, Mutex<AppService>>,
    query: String,
) -> Result<SearchResponse, String> {
    let now = Instant::now();
    let res = state
        .lock()
        .await
        .search(query)
        .await
        .map_err(|e| e.to_string());
    let elapsed = now.elapsed();
    log::info!("Searching took {}ms", elapsed.as_millis());

    res
}
