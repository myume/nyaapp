use tauri::State;
use tokio::sync::Mutex;

use crate::app_service::{AppService, SearchResult};

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
) -> Result<Vec<SearchResult>, String> {
    state
        .lock()
        .await
        .search(query)
        .await
        .map_err(|e| e.to_string())
}
