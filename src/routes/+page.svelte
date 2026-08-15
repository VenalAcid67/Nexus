<script lang="ts">
  import { ping } from "$lib/tauri-api";

  let result = "";

  async function handlePing() {
    result = await ping();
  }
</script>

<main class="container">
  <h1>Nexus</h1>
   <button on:click={handlePing}>Ping</button>
    {#if result}
      <p>{result}</p>
    {/if}
</main>

<style>
  .container {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    height: 100vh;
    gap: 1rem;
  }
</style>
