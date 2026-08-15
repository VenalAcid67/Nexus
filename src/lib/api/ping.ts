import { invoke } from "@tauri-apps/api/core";

export async function ping(): Promise<string> {
  return await invoke("ping");
}
