<script lang="ts">
  import { subscribe } from "../ws";

  let testHtml = "";
  subscribe((data) => {
    testHtml = testHtml +  data;
  });
</script>

<div class="content-container">
  <iframe class="content" title="document" srcdoc={testHtml} />
</div>

<style>
  .content-container {
    all: initial;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }

  .content {
    display: block;
    background: white;
    margin: 2rem;
    margin-top: 1rem;
    width: 100%;
    height: 100%;
    min-height: 80rem;
    border: 1px solid var(--background-disabled);
  }

  @media (min-width: 768px) {
    .content {
      margin-left: 8rem;
      margin-right: 8rem;
    }
  }
  @media (min-width: 1024px) {
    .content {
      margin-left: 16rem;
      margin-right: 16rem;
    }
  }

  @media (min-width: 1536px) {
    .content {
      margin-left: 24rem;
      margin-right: 24rem;
    }
  }
</style>
