<script>
	import '../app.css';
	import { messageStore } from '../websocket';
	import Bar from './bar.svelte';

	messageStore.subscribe;
</script>

<main class="flex flex-col h-screen items-center">
	<Bar />
	<iframe
		class="my-4 w-full max-w-container h-full"
		title="Markdown Preview Server Content"
		src="/content"
		sandbox="allow-scripts allow-same-origin"
	/>
</main>
