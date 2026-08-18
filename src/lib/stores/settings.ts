import { writable } from "svelte/store";

export type ProviderName = "ollama" | "anthropic" | "openrouter";

export interface ProviderSettings {
  provider: ProviderName;
  model: string;
  apiKey: string;
}

const defaults: Record<ProviderName, { model: string }> = {
  ollama: { model: "llama3.2" },
  anthropic: { model: "claude-3-5-haiku-20241022" },
  openrouter: { model: "meta-llama/llama-3.3-70b-instruct:free" },
};

function createSettingsStore() {
  const { subscribe, update } = writable<ProviderSettings>({
    provider: "ollama",
    model: defaults.ollama.model,
    apiKey: "",
  });

  return {
    subscribe,
    setProvider: (provider: ProviderName) =>
      update((s) => ({ ...s, provider, model: defaults[provider].model, apiKey: "" })),
    setModel: (model: string) => update((s) => ({ ...s, model })),
    setApiKey: (apiKey: string) => update((s) => ({ ...s, apiKey })),
  };
}

export const settings = createSettingsStore();
