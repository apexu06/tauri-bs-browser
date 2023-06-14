<script lang="ts">
	import './styles.css';
	import Hamburger from './components/hamburger.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let query: string;

	let data = [];
	async function searchMaps() {
		await invoke('get_maps', { query: query, page: 0 }).then((res) => {
			data = res as [];
			console.log(JSON.stringify(data));
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
		<button on:click={searchMaps}>Search</button>
	</div>

	<div class="flex flex-col items-center">
		{#each data as map}
			<div class="flex flex-col items-center">
				<h3>{map.name}</h3>
				<h4>{map.description}</h4>
			</div>
		{/each}
	</div>
</main>

<style>
</style>
