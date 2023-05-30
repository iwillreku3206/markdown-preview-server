<script lang="ts">
	import { onMount } from 'svelte';
	import { messageStore } from '../../websocket';
	import { BYTES_CSS, BYTES_DATA } from '../../websocketPrefixes';
	import('https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js');

	let content = '';
	let css = '';

	let styleElm: HTMLSpanElement;
	let contentElm: HTMLElement;

	onMount(() => {
		messageStore.subscribe(async (message) => {
			if (!message) return;
			const buf = await message.arrayBuffer();
			const bytes = new Uint8Array(buf);
			const magicBytes = bytes.slice(0, 4).join('');
			if (magicBytes === BYTES_DATA) {
				content = new TextDecoder().decode(bytes.slice(4));
				contentElm.innerHTML = content;
				if (navigator.userAgent.includes('Chrome') || !window.MathMLElement) {
					// eslint-disable-next-line @typescript-eslint/no-explicit-any
					(window as any).MathJax.typeset();
				}
			}

			if (magicBytes === BYTES_CSS) {
				css = new TextDecoder().decode(bytes.slice(4));
				styleElm.innerHTML = `<style>${css}</style>`;
			}
		});
	});
</script>

<main>
	<span bind:this={styleElm} />
	<main bind:this={contentElm} />
</main>

<style>
	main {
		word-wrap: break-word;
		overflow-wrap: break-word;
    word-break: break-all;
	}
</style>
