<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { sendMessage, onChatChunk, type ChatMessage } from "$lib/api";

  let messages: ChatMessage[] = [];
  let input = "";
  let sending = false;
  let unlisten: (() => void) | undefined;

  onMount(async () => {
    unlisten = await onChatChunk((chunk) => {
      const last = messages[messages.length - 1];
      if (last && last.role === "assistant") {
        last.content += chunk;
        messages = [...messages];
      }
    });
  });

  onDestroy(() => unlisten?.());

  async function handleSend() {
    if (!input.trim() || sending) return;

    messages = [...messages, { role: "user", content: input }, { role: "assistant", content: "" }];
    input = "";
    sending = true;

    try {
      await sendMessage("ollama", "gemma3:4b", messages.slice(0, -1));
    } catch (err) {
      messages[messages.length - 1].content = `Error: ${err}`;
      messages = [...messages];
    } finally {
      sending = false;
    }
  }
</script>

<div class="chat">
  {#each messages as msg}
    <div class="message {msg.role}">
      <strong>{msg.role === "user" ? "You" : "Nexus"}:</strong>
      <span>{msg.content}</span>
    </div>
  {/each}
</div>

<form on:submit|preventDefault={handleSend} class="input-row">
  <input bind:value={input} placeholder="Ask Nexus..." disabled={sending} />
  <button type="submit" disabled={sending}>Send</button>
</form>

<style>
  .chat { width: 100%; max-width: 600px; flex: 1; overflow-y: auto; text-align: left; }
  .message { margin-bottom: 0.75rem; }
  .message.user { color: #0b5fa5; }
  .message.assistant { color: #111111; }
  .input-row { display: flex; width: 100%; max-width: 600px; gap: 0.5rem; }
  .input-row input { flex: 1; padding: 0.5rem; }
</style>
