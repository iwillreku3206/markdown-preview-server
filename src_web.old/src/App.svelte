<script lang="ts">
  import Content from "./lib/Content.svelte";

  let dark = true;
  let followBottom = false;

  function toggleDarkMode() {
    dark = !dark;
  }

  function toggleFollow() {
    followBottom = !followBottom;
  }
</script>

<main class={dark ? "dark" : ""}>
  <div class="topbar">
    <p class="topbar-text">Markdown Preview Server</p>
    <p class="topbar-center">File Name</p>
    <div class="topbar-right">
      <button on:click={toggleFollow} class="topbar-button">
        <spawn class="material-symbols-outlined">
          {followBottom ? "arrow_downward" : "arrow_upward"}
        </spawn>
      </button>
      <button on:click={toggleDarkMode} class="topbar-button">
        <span class="material-symbols-outlined">
          {!dark ? "dark_mode" : "light_mode"}
        </span>
      </button>
    </div>
  </div>
  <Content {followBottom} />
</main>

<style>
  main {
    background: var(--background);
    color: var(--text-primary);
    min-height: 100vh;
  }

  .topbar {
    max-width: 100%;
    padding: 0.5rem;
    padding-left: 1rem;
    padding-right: 1rem;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    user-select: none;
    position: sticky;
    background: var(--background-secondary);
    color: var(--text-on-primary);
    top: 0;
    left: 0;
  }

  .topbar-button {
    color: var(--text-on-primary);
    background: none;
    border: none;
  }

  .topbar-center {
    text-align: center;
    font-size: 1.25rem;
    font-weight: 500;
    margin: 0;
    align-self: center;
  }

  .topbar-right {
    margin-left: auto;
    text-align: right;
  }

  .topbar-text {
    font-size: 1.25rem;
    font-weight: 500;
    margin: 0;
    align-self: center;
  }
</style>
