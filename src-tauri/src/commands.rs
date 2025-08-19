use std::time::{Duration, Instant};

use tauri::{Emitter, State};
use tokio::sync::Mutex;

use crate::{
    app_service::{AppService, SearchResponse},
    library::{LibraryEntry, LibraryEntrySettings},
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
pub async fn load_cbz(
    state: State<'_, Mutex<AppService>>,
    id: String,
    file_num: usize,
) -> Result<usize, String> {
    let num_pages = state
        .lock()
        .await
        .load_cbz(&id, file_num)
        .await
        .map_err(|e| e.to_string())?;

    Ok(num_pages)
}

#[tauri::command]
pub async fn update_reading_progress(
    state: State<'_, Mutex<AppService>>,
    id: String,
    file_num: usize,
    updated_page: usize,
) -> Result<(), String> {
    state
        .lock()
        .await
        .update_reading_progress(&id, file_num, updated_page)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_dimensions(
    state: State<'_, Mutex<AppService>>,
    id: String,
    file_num: usize,
) -> Result<Vec<(u32, u32)>, String> {
    state
        .lock()
        .await
        .get_dimensions(&id, file_num)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_library_entry_settings(
    state: State<'_, Mutex<AppService>>,
    id: String,
    settings: LibraryEntrySettings,
) -> Result<(), String> {
    state
        .lock()
        .await
        .update_library_entry_settings(&id, settings)
        .await
        .map_err(|e| e.to_string())
}
