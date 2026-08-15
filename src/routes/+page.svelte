<script lang="ts">
  import Chat from "$lib/components/Chat.svelte";
</script>

<main class="container">
  <h1>Nexus</h1>
  <Chat />
</main>

<style>
  .container {
    margin: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    height: 100vh;
    padding: 2rem;
    box-sizing: border-box;
    gap: 1rem;
  }
</style>
