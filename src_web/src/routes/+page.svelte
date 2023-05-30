<script lang="ts">
	import '../app.css';
	import { messageStore } from '../websocket';
	import Bar from './bar.svelte';
</script>

<main class="flex flex-col h-screen items-center bg-base-200">
	<Bar />
	<iframe
		class="mx-8 my-8 w-full max-w-container-small lg:max-w-container h-full"
		title="Markdown Preview Server Content"
		src="/content"
		sandbox="allow-scripts allow-same-origin"
	/>
</main>

<style>
	main {
		max-width: 100vw;
	}
</style>
