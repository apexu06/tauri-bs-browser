<script lang="ts">
	import { onMount } from 'svelte';
	import customSelect from 'custom-select';

	export let searchType: string = 'Latest';

	let searchSelect: HTMLSelectElement;

	onMount(() => {
		customSelect(searchSelect);
	});
</script>

<div class="my-6 flex w-5/6 rounded-lg bg-neutral-900 p-2">
	<div class="flex items-center">
		<label for="searchSelect" class="mr-3">Search by: </label>
		<select
			name="searchSelect"
			id="searchSelect"
			bind:this={searchSelect}
			on:change={() => (searchType = searchSelect.value)}
		>
			<option value="Latest">Latest</option>
			<option value="Relevance">Relevance</option>
			<option value="Rating">Rating</option>
			<option value="Curated">Curated</option>
		</select>
	</div>
</div>
