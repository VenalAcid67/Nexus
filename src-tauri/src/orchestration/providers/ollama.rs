use async_trait::async_trait;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::orchestration::provider_client::{ChatMessage, ChatProvider};

#[derive(Default)]
pub struct OllamaProvider;

#[derive(Debug, Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaChatChunk {
    message: Option<ChatMessage>,
    done: bool,
}

#[async_trait]
impl ChatProvider for OllamaProvider {
    async fn send_chat(
        &self,
        app: &AppHandle,
        model: &str,
        messages: &[ChatMessage],
    ) -> Result<String, String> {
        let client = reqwest::Client::new();
        let body = OllamaChatRequest { model, messages, stream: true };

        let response = client
            .post("http://localhost:11434/api/chat")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to reach Ollama: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("Ollama returned status {}", response.status()));
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
                if line.is_empty() { continue; }

                let parsed: OllamaChatChunk = serde_json::from_str(&line)
                    .map_err(|e| format!("Failed to parse Ollama response: {e}"))?;

                if let Some(msg) = &parsed.message {
                    full_reply.push_str(&msg.content);
                    app.emit("chat-chunk", msg.content.clone())
                        .map_err(|e| format!("Failed to emit event: {e}"))?;
                }

                if parsed.done { break; }
            }
        }

        Ok(full_reply)
    }
}
