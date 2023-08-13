<script lang="ts">
	import { onMount } from 'svelte';
	import { messageStore, send } from '../../websocket';
	import { BYTES_CSS, BYTES_DATA } from '../../websocketPrefixes';
	import('https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js');

	function jumpto(anchor: string) {
		window.location.href = '#' + anchor;
	}

	let content = '';
	let css = '';

	let styleElm: HTMLSpanElement;
	let contentElm: HTMLElement;
	let followBottom =
		localStorage.getItem('markdown-preview-server__options__followBottom') === 'true';

	onMount(() => {
		messageStore.subscribe(async (message) => {
			if (!message) return;
			const buf = await message.arrayBuffer();
			const bytes = new Uint8Array(buf);
			const magicBytes = bytes.slice(0, 4).join('');
			if (magicBytes === BYTES_DATA) {
				content = new TextDecoder().decode(bytes.slice(4));
				if (contentElm) {
					contentElm.innerHTML = content;
				}
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

	function handleClick(e: MouseEvent) {
		if (e.target && e.target instanceof HTMLAnchorElement) {
			const path = e.target.getAttribute('data-path');
			if (path) {
				e.preventDefault();
				const pathBytes = new TextEncoder().encode(path);
				const buf = new Uint8Array(4 + pathBytes.length);

				buf[0] = 0x00;
				buf[1] = 0x00;
				buf[2] = 0x01; // 0x01 = USER_INPUT
				buf[3] = 0x00; // 0x00 = GOTO_FILE

        buf.set(pathBytes, 4);

				send(buf);
				return;
			}
		}
	}
</script>

<svelte:window on:click={handleClick} />

<main class="content-main">
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
