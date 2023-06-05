<script lang="ts">
	import { messageStore } from '../websocket';
	import { BYTES_FILENAME, BYTES_FRONTMATTER } from '../websocketPrefixes';
	import IconSettings from '~icons/mdi/settings';
	import IconDownArrow from '~icons/mdi/arrow-down';
	import { options } from '../optionStore';

	let title = 'Untitled';
	let fileName = '';

	let followBottom =
		localStorage.getItem('markdown-preview-server__options__followBottom') === 'true';

	$: {
		options.update((options) => {
			options.followBottom = followBottom;
			return { ...options };
		});
	}

	messageStore.subscribe(async (message) => {
		if (!message) return;
		const buf = await message.arrayBuffer();
		const bytes = new Uint8Array(buf);
		const magicBytes = bytes.slice(0, 4).join('');
		if (magicBytes === BYTES_FILENAME) {
			fileName = new TextDecoder().decode(bytes.slice(4));
		}

		if (magicBytes === BYTES_FRONTMATTER) {
			try {
				const frontmatter = new TextDecoder().decode(bytes.slice(4));
				const parsedFrontmatter = JSON.parse(frontmatter);
				parsedFrontmatter.title && (title = parsedFrontmatter.title);
			} catch (e) {
				console.error('Invalid Frontmatter: ', e);
			}
		}
	});
</script>

<div class="flex flex-row h-16 bg-base-300 items-center px-4 w-full">
	<h6 class="flex-1 flex justify-start items-center text-xl font-semibold">
		Markdown Preview Server
	</h6>
	<h6 class="flex-1 flex justify-center items-center">
		<div class="flex flex-col text-center">
			<div class="text-xl font-bold">{title}</div>
			<div class="text-sm">{fileName}</div>
		</div>
	</h6>
	<div class="flex-1 justify-end items-center">
		<div class="flex flex-row-reverse">
			<button
				class={`${followBottom ? 'text-success' : 'text-base-content'} text-xl`}
				on:click={() => (followBottom = !followBottom)}><IconDownArrow /></button
			>
			<label for="options-modal" class="btn btn-ghost text-xl"><IconSettings /></label>
		</div>
	</div>
</div>
