use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[async_trait]
pub trait ChatProvider {
    async fn send_chat(
        &self,
        app: &AppHandle,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<String, String>;
}

pub fn get_provider(
    provider_name: &str,
    api_key: Option<String>,
) -> Result<Box<dyn ChatProvider + Send + Sync>, String> {
    match provider_name {
        "ollama" => Ok(Box::new(super::providers::ollama::OllamaProvider::default())),
        "anthropic" => {
            let key = api_key
                .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                .ok_or_else(|| {
                    "Missing Anthropic API key. Set ANTHROPIC_API_KEY or pass one.".to_string()
                })?;
            Ok(Box::new(super::providers::anthropic::AnthropicProvider::new(key)))
        }
	"openrouter" => {
	    let key = api_key
		.or_else(|| std::env::var("OPENROUTER_API_KEY").ok())
		.ok_or_else(|| "Missing OpenRouter API key. ".to_string())?;
	    Ok(Box::new(super::providers::openrouter::OpenRouterProvider::new(key)))
	}
        other => Err(format!("Unknown provider: {other}")),
    }
}
