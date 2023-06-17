<script lang="ts">
	import { onMount } from 'svelte';
	import Hamburger from './components/common/Hamburger.svelte';
	import MapTable from './components/map-table/MapTable.svelte';
	import type { MapDetail } from './types/MapDetail';
	import { invoke } from '@tauri-apps/api/tauri';
	import FilterBar from './components/filter-bar/FilterBar.svelte';

	let query = '';
	let maps: MapDetail[] = [];
	let error = '';

	onMount(() => {
		searchMaps();
	});

	let sortOrder = 'Latest';
	let onlyRanked = false;
	let onlyVerified = false;
	let onlyCurated = false;
	let bpmValues: [number, number];

	function searchMaps() {
		maps = [];
		let filters = [
			{
				name: 'ranked',
				active: onlyRanked,
			},
			{
				name: 'verified',
				active: onlyVerified,
			},
			{
				name: 'curated',
				active: onlyCurated,
			},
		];
		invoke('get_maps', {
			query: query,
			page: 0,
			sortMode: sortOrder,
			filters: filters,
			minBpm: bpmValues[0],
			maxBpm: bpmValues[1],
			currentMaps: maps,
		})
			.then((res) => {
				maps = res as MapDetail[];
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
		<input
			bind:value={query}
			type="text"
			placeholder="map title"
			class="rounded-l-xl"
		/>
		<button
			class="rounded-r-xl hover:rounded-xl"
			on:click={() => searchMaps()}>Search</button
		>
	</div>

	<div class="flex w-full flex-col items-center">
		<FilterBar
			bind:searchType={sortOrder}
			bind:onlyRanked
			bind:onlyCurated
			bind:onlyVerified
			bind:bpmValues
		/>
		<MapTable {maps} />
		<span class="text-center text-2xl font-extrabold text-red-700"
			>{error}</span
		>
	</div>
</main>
