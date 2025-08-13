use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use base64::{engine::general_purpose, Engine as _};
use tauri::{Emitter, State};
use tokio::sync::Mutex;

use crate::{
    app_service::{AppService, SearchResponse},
    library::LibraryEntry,
    torrent::TorrentStats,
};

#[tauri::command]
pub async fn download(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppService>>,
    id: String,
) -> Result<(), String> {
    app_handle
        .emit(
            "download-started",
            (
                id.clone(),
                state
                    .lock()
                    .await
                    .get_title_by_id(&id)
                    .await
                    .unwrap_or("".to_owned()),
            ),
        )
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

#[tauri::command]
pub async fn list_torrents(
    state: State<'_, Mutex<AppService>>,
) -> Result<Vec<TorrentStats>, String> {
    Ok(state.lock().await.list_torrents().await)
}

#[tauri::command]
pub async fn toggle_pause(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppService>>,
    id: String,
) -> Result<(), String> {
    state
        .lock()
        .await
        .toggle_pause(&id)
        .await
        .map_err(|e| e.to_string())?;

    if let Ok(mut rx) = state.lock().await.get_torrent_stats_receiver(&id).await {
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
    }

    Ok(())
}

#[tauri::command]
pub async fn list_library(
    state: State<'_, Mutex<AppService>>,
) -> Result<Vec<LibraryEntry>, String> {
    Ok(state.lock().await.fetch_library().await)
}

#[tauri::command]
pub async fn remove_download(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppService>>,
    id: String,
) -> Result<(), String> {
    app_handle
        .emit("download-removed", id.clone())
        .map_err(|e| e.to_string())?;

    state
        .lock()
        .await
        .remove_download(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete(
    app_handle: tauri::AppHandle,
    state: State<'_, Mutex<AppService>>,
    id: String,
) -> Result<(), String> {
    app_handle
        .emit("download-removed", id.clone())
        .map_err(|e| e.to_string())?;

    state
        .lock()
        .await
        .delete(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn read_cbz(app_handle: tauri::AppHandle, path: String) -> Result<(), String> {
    log::info!("Attempting to read CBZ file at: {}", path);
    let data = match crate::utils::read_cbz(&PathBuf::from(&path)).await {
        Ok(data) => data,
        Err(e) => {
            log::error!("Error reading CBZ file at {}: {}", path, e);
            return Err(e.to_string());
        }
    };
    log::info!(
        "Successfully read {} files from CBZ at: {}",
        data.len(),
        path
    );

    let encoded_data = data
        .into_iter()
        .map(|d| general_purpose::STANDARD.encode(d));

    for page in encoded_data {
        app_handle
            .emit("page-read", page)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
