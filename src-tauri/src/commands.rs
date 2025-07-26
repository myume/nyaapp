use std::time::Instant;

use tauri::State;
use tokio::sync::Mutex;

use crate::app_service::{AppService, SearchResponse};

#[tauri::command]
pub async fn download(state: State<'_, Mutex<AppService>>, id: String) -> Result<(), String> {
    state
        .lock()
        .await
        .download(id)
        .await
        .map_err(|e| e.to_string())
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
