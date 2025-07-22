use anyhow::Result;

#[tauri::command]
pub async fn init() -> Result<()> {
    Ok(())
}
