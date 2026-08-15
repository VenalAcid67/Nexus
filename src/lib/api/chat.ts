import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface ChatMessage {
  role: "user" | "assistant";
  content: string;
}

export async function sendMessage(model: string, messages: ChatMessage[]): Promise<string> {
  return await invoke("send_message", { model, messages });
}

export function onChatChunk(callback: (chunk: string) => void): Promise<UnlistenFn> {
  return listen<string>("chat-chunk", (event) => callback(event.payload));
}
