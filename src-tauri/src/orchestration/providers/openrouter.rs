use async_trait::async_trait;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::orchestration::provider_client::{ChatMessage, ChatProvider};

pub struct OpenRouterProvider {
    api_key: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Debug, Serialize)]
struct OpenAiChatRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAiStreamChunk {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    delta: OpenAiDelta,
}

#[derive(Debug, Deserialize)]
struct OpenAiDelta {
    content: Option<String>,
}

#[async_trait]
impl ChatProvider for OpenRouterProvider {
    async fn send_chat(
        &self,
        app: &AppHandle,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<String, String> {
        let client = reqwest::Client::new();
        let body = OpenAiChatRequest { model, messages, stream: true };

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to reach OpenRouter: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("OpenRouter returned {status}: {text}"));
        }

        let mut stream = response.bytes_stream();
        let mut full_reply = String::new();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Stream error: {e}"))?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer.drain(..=pos);

                let Some(data) = line.strip_prefix("data: ") else { continue };
                if data == "[DONE]" { continue; }
                if data.is_empty() { continue; }

                if let Ok(parsed) = serde_json::from_str::<OpenAiStreamChunk>(data) {
                    if let Some(choice) = parsed.choices.first() {
                        if let Some(text) = &choice.delta.content {
                            full_reply.push_str(text);
                            app.emit("chat-chunk", text.clone())
                                .map_err(|e| format!("Failed to emit event: {e}"))?;
                        }
                    }
                }
            }
        }

        Ok(full_reply)
    }
}
