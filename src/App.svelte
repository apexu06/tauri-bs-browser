<script lang="ts">
	import { onMount } from 'svelte';
	import Hamburger from './components/Hamburger.svelte';
	import MapTable from './components/map-table/MapTable.svelte';
	import type { Map } from './types/Map';
	import { invoke } from '@tauri-apps/api/tauri';
	import FilterBar from './components/FilterBar.svelte';

	let query = '';
	let maps: Map[] = [];
	let error = '';

	onMount(() => {
		searchMaps();
	});

	let sortOrder = 'Latest';

	function searchMaps() {
		maps = [];
		invoke('get_maps', {
			query: query,
			page: 0,
			sortMode: sortOrder,
			currentMaps: maps,
		})
			.then((res) => {
				maps = res as Map[];
			})
			.catch((err) => {
				error = 'Something went wrong: ' + err;
			});
	}
</script>

<main class="flex h-full w-full flex-col items-center justify-center">
	<div class="m-8 h-fit w-fit self-start">
		<Hamburger />
	</div>

	<div class="flex flex-col items-center">
		<h1>BeatSaber Browser</h1>
		<h3 class="mb-4">Search Maps</h3>
	</div>

	<div class="flex h-12 w-2/5 shadow shadow-black">
		<input bind:value={query} type="text" placeholder="map title" />
		<button on:click={() => searchMaps()}>Search</button>
	</div>

	<div class="flex w-full flex-col items-center">
		<FilterBar bind:searchType={sortOrder} />
		<MapTable {maps} />
		<span class="text-center text-2xl font-extrabold text-red-700"
			>{error}</span
		>
	</div>
</main>

<style>
</style>
