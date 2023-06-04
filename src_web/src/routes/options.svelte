<script lang="ts">
	import { availableThemes, options } from '../optionStore';
	let currentTheme = $options.theme;

	$: {
		options.update((options) => {
			options.theme = currentTheme;
			return { ...options };
		});
	}
</script>

<input type="checkbox" id="options-modal" class="modal-toggle" />
<div class="modal">
	<div class="modal-box flex flex-col gap-4">
		<h5 class="text-xl font-bold">Options</h5>

		<div class="form-control">
			<label class="label" for="options-theme-select">
				<span class="label-text">Theme</span>
			</label>
			<select
				name="theme"
				bind:value={currentTheme}
				class="select select-bordered"
				id="options-theme-select"
			>
				{#each availableThemes as theme}
					<option value={theme}>{theme}</option>
				{/each}
			</select>
		</div>

		<hr />
		<div class="modal-actions">
			<label for="options-modal" class="btn">Close</label>
		</div>
	</div>
</div>
