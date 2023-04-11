<script lang="ts">
  import { scrollToBottom } from "../util/scrollToBottom";

  export let followBottom: boolean;

  import { subscribe } from "../ws";

  let iframe: HTMLIFrameElement;

  let iframeHeight =
    iframe?.contentWindow?.document.getElementsByTagName("html").item(0)
      .scrollHeight || document.body.scrollHeight;

  subscribe((data) => {
    if (iframe && iframe.contentWindow.document.body) {
      iframe.contentWindow.document.body.innerHTML = data;
      iframeHeight = iframe.contentWindow.document
        .getElementsByTagName("html")
        .item(0).scrollHeight;
      if (followBottom) {
        scrollToBottom();
      }
    }
  });
</script>

<div class="content-container">
  <div />
  <iframe
    bind:this={iframe}
    class="content"
    title="document"
    height={iframeHeight}
  />
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
    background: white;
    margin: 2rem;
    margin-top: 1rem;
    width: 100%;
    border: 1px solid var(--background-disabled);
    padding: none;
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
