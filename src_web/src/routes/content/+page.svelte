<script lang="ts">
	import { onMount } from 'svelte';
	import { messageStore } from '../../websocket';
	import { BYTES_CSS, BYTES_DATA } from '../../websocketPrefixes';
	import('https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js');

	function jumpto(anchor: string) {
		window.location.href = '#' + anchor;
	}

	let content = '';
	let css = '';

	let styleElm: HTMLSpanElement;
	let contentElm: HTMLElement;
	let followBottom = localStorage.getItem('markdown-preview-server__options__followBottom') === 'true';

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
				if (followBottom) jumpto('bottom');
			}

			if (magicBytes === BYTES_CSS) {
				css = new TextDecoder().decode(bytes.slice(4));
				styleElm.innerHTML = `<style>${css}</style>`;
			}
		});
	});

	addEventListener('message', (event) => {
		if (event.data === 'followBottom=TRUE') {
			followBottom = true;
		}
		if (event.data === 'followBottom=FALSE') {
			followBottom = false;
		}
	});
</script>

<main>
	<span bind:this={styleElm} />
	<main bind:this={contentElm} />
	<!-- svelte-ignore a11y-missing-content -->
	<a id="bottom" />
</main>

<style>
	main {
		background-color: white;
		word-wrap: break-word;
		overflow-wrap: break-word;
		word-break: break-all;
	}
</style>
