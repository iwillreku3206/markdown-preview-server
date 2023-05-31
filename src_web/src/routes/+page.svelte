<script lang="ts">
	import '../app.css';
	import Bar from './bar.svelte';
	import { options } from '../optionStore';
	import Options from './options.svelte';
</script>

<Options />
<main data-theme={$options.theme} class="flex flex-col h-screen items-center bg-base-200">
	<Bar />
	<iframe
		class="mx-8 my-8 w-full max-w-container-small lg:max-w-container h-full bg-white"
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
