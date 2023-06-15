<script lang="ts">
	import { onMount } from 'svelte';

	export let searchType: string = 'Latest';

	let searchSelect: HTMLSelectElement;
</script>

<div class="my-6 flex w-5/6 rounded-lg bg-neutral-900 p-2">
	<div>
		<label for="searchSelect">Search by: </label>
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

<style>
	select option {
		background-color: #ff0000;
	}

	select {
		color: black;
	}
</style>
