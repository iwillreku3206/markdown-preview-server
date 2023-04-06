<script lang="ts">
  import Counter from "./lib/Counter.svelte";
</script>

<main>
  <div class="topbar">Markdown Preview Server</div>
  <div class="content" />
</main>

<style>
  main {
    background: var(--background);
    text: var(--text-primary);
  }

  .topbar {
    width: 100%;
    padding: 0.5rem;
    border-bottom: 0.1rem solid var(--background-disabled);
  }

  .content {
    all: unset;
  }
</style>
