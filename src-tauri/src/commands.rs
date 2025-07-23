use tauri::State;
use tokio::sync::Mutex;

use crate::app_service::AppService;

#[tauri::command]
pub async fn download(state: State<'_, Mutex<AppService>>, id: String) -> Result<(), String> {
    state
        .lock()
        .await
        .download(id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
