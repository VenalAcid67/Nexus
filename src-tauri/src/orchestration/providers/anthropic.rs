use async_trait::async_trait;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::orchestration::provider_client::{ChatMessage, ChatProvider};

pub struct AnthropicProvider {
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Debug, Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    messages: &'a [ChatMessage],
    stream: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AnthropicStreamEvent {
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { delta: Delta },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct Delta {
    text: Option<String>,
}

#[async_trait]
impl ChatProvider for AnthropicProvider {
    async fn send_chat(
        &self,
        app: &AppHandle,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<String, String> {
        let client = reqwest::Client::new();
        let body = AnthropicRequest { model, max_tokens: 1024, messages, stream: true };

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to reach Anthropic: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Anthropic returned {status}: {text}"));
        }

        let mut stream = response.bytes_stream();
        let mut full_reply = String::new();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Stream error: {e}"))?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(pos) = buffer.find("\n\n") {
                let event_block = buffer[..pos].to_string();
                buffer.drain(..pos + 2);

                for line in event_block.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(AnthropicStreamEvent::ContentBlockDelta { delta }) =
                            serde_json::from_str::<AnthropicStreamEvent>(data)
                        {
                            if let Some(text) = delta.text {
                                full_reply.push_str(&text);
                                app.emit("chat-chunk", text)
                                    .map_err(|e| format!("Failed to emit event: {e}"))?;
                            }
                        }
                    }
                }
            }
        }

        Ok(full_reply)
    }
}
