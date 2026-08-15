use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    provider: String,
    model: String,
    api_key: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<String, String> {
    let provider = get_provider(&provider, api_key)?;
    provider.send_chat(&app, &model, &messages).await
}
