<script lang="ts">
	import '../app.css';
	import Bar from './bar.svelte';
	import { options } from '../optionStore';
	import Options from './options.svelte';
	import { onMount } from 'svelte';
	import { resetWebSocket } from '../websocket';

	let iframe: HTMLIFrameElement;
	onMount(() => {
		options.subscribe((options) => {
			if (!iframe || !iframe.contentWindow) return;
			iframe.contentWindow.postMessage(
				options.followBottom ? 'followBottom=TRUE' : 'followBottom=FALSE',
				'*'
			);
		});
	});
	function onResetWebSocket() {
		if (!iframe || !iframe.contentWindow) return;
		iframe.contentWindow.postMessage('wsReconnect', '*');
		resetWebSocket();
	}
</script>

<Options />
<main data-theme={$options.theme} class="flex flex-col h-screen items-center bg-base-200">
	<Bar resetWebSocket={onResetWebSocket} />
	<iframe
		class="mx-8 my-8 w-full max-w-container-small lg:max-w-container h-full bg-white"
		title="Markdown Preview Server Content"
		src="/content"
		sandbox="allow-scripts allow-same-origin"
		bind:this={iframe}
	/>
</main>

<style>
	main {
		max-width: 100vw;
	}
</style>
