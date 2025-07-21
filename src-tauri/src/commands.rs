use crate::metadata::mangabaka::Mangabaka;
use anyhow::Result;

#[tauri::command]
pub async fn init() -> Result<()> {
    // Mangabaka::download_db().await?;
    Ok(())
}
