import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface ChatMessage {
  role: "user" | "assistant";
  content: string;
}

export async function sendMessage(
	provider: string,
	model: string, messages: ChatMessage[],
	apiKey?: string
): Promise<string> {
  return await invoke("send_message", { provider, model, apiKey, messages });
}

export function onChatChunk(callback: (chunk: string) => void): Promise<UnlistenFn> {
  return listen<string>("chat-chunk", (event) => callback(event.payload));
}
