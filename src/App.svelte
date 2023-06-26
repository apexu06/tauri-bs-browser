<script lang="ts">
	import { onMount } from 'svelte';
	import Hamburger from './components/common/Hamburger.svelte';
	import MapTable from './components/map-table/MapTable.svelte';
	import type { MapDetail } from './types/MapDetail';
	import FilterBar from './components/filter-bar/FilterBar.svelte';
	import './styles.css';
	import { fetchMaps } from './functions/request';

	let searchButton: HTMLButtonElement;
	let searchButtonWidth: number = searchButton?.clientWidth;

	let query = '';
	let maps: MapDetail[] = [];
	let error = '';
	let sortOrder = 'Latest';
	let onlyRanked = false;
	let onlyVerified = false;
	let onlyCurated = false;
	let bpmValues: [number, number];
	let startDate: Date;
	let endDate: Date;

	onMount(() => {
		searchButton.addEventListener('mouseenter', () => {
			searchButtonWidth = searchButton?.clientWidth;
		});
	});

	async function getMaps(current_maps: MapDetail[], page: number) {
		const response = await fetchMaps(
			{
				query,
				page,
				sortOrder,
				filters: [
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
				],
				minBpm: bpmValues[0],
				maxBpm: bpmValues[1],
				startDate,
				endDate,
			},
			current_maps
		);

		try {
			maps = response;
		} catch (e) {
			error = e;
		}
	}

	function getMock() {
		maps[0] = {
			id: 'abc123',
			name: 'Map Name',
			lastPublishedAt: '2023-06-19T12:34:56Z',
			curatedAt: null,
			metadata: {
				songName: 'Song Name',
				songSubName: 'Sub Name',
				songAuthorName: 'Author Name',
				levelAuthorName: 'Level Author',
				bpm: 120.5,
				duration: 180,
			},
			stats: {
				downvotes: 10,
				upvotes: 100,
			},
			description: 'Map Description',
			ranked: true,
			qualified: false,
			versions: [
				{
					downloadURL: 'https://example.com/map/download',
					previewURL: 'https://example.com/map/preview',
					coverURL:
						'https://eu.cdn.beatsaver.com/25b1354d0eef8b1d0fb0d82e73302a0c81845351.jpg',
					diffs: [
						{
							notes: 100,
							bombs: 5,
							characteristic: 'Standard',
							difficulty: 'Expert',
							njs: 12.0,
							nps: 6.0,
						},
					],
					hash: 'abcdef123456',
				},
			],
			automapper: false,
			uploader: {
				verifiedMapper: true,
			},
		};
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

	<div class="flex h-12 w-2/5">
		<input
			bind:value={query}
			type="text"
			placeholder="map title"
			class="rounded-l-xl"
		/>
		<button
			class="rounded-r-xl hover:rounded-xl"
			bind:this={searchButton}
			style="--buttonWidth: {searchButtonWidth}px"
			on:click={() => {
				maps = [];
				getMaps(maps, 0);
			}}>Search</button
		>
	</div>

	<div class="flex w-full flex-col items-center">
		<FilterBar
			bind:searchType={sortOrder}
			bind:onlyRanked
			bind:onlyCurated
			bind:onlyVerified
			bind:bpmValues
			bind:startDate
			bind:endDate
		/>
		<MapTable {maps} {getMaps} />
		<span class="text-center text-2xl font-extrabold text-red-700"
			>{error}</span
		>
	</div>
</main>

<style>
	button {
		height: 100%;
		background: linear-gradient(45deg, var(--primary), var(--secondary));
		padding: 4px;
		font-weight: bold;
		font-size: 120%;
		width: 30%;
		transition: 400ms;
	}

	button:hover {
		transition: 400ms;
		background-position: var(--buttonWidth);
	}
</style>
