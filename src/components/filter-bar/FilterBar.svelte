<script lang="ts">
	import { onMount } from 'svelte';
	import customSelect from 'custom-select';
	import ToggleSwitch from '../common/ToggleSwitch.svelte';
	import RangeSlider from 'svelte-range-slider-pips';

	export let searchType = 'Latest';
	export let onlyRanked = false;
	export let onlyVerified = false;
	export let onlyCurated = false;

	export let bpmValues = [0, 500];

	let searchSelect: HTMLSelectElement;
	let sortSelect: HTMLSelectElement;

	onMount(() => {
		customSelect(searchSelect);
		customSelect(sortSelect);
	});

	$: console.log(bpmValues);
</script>

<div class="my-6 flex w-5/6 items-center rounded-lg bg-neutral-900 p-2">
	<div
		class="mr-4 flex items-center border-r border-r-white pl-2 pr-4 text-center"
	>
		<select
			name="sortSelect"
			id="sortSelect"
			bind:this={searchSelect}
			on:change={() => (searchType = searchSelect.value)}
		>
			<option value="Latest">Latest</option>
			<option value="Relevance">Relevance</option>
			<option value="Rating">Rating</option>
			<option value="Curated">Curated</option>
		</select>
	</div>

	<div class="mr-4 flex items-center border-r border-r-white pr-4">
		<span class="text-xl font-bold">Filters</span>

		<div
			class="ml-2 flex w-full items-center rounded-lg bg-neutral-800 px-2 py-1"
		>
			<div class="mr-2 border-r border-r-neutral-600 pr-2">
				<span class="mr-2">Ranked: </span>
				<ToggleSwitch bind:checked={onlyRanked} color="var(--ranked)" />
			</div>
			<div class="mr-2 border-r border-r-neutral-600 pr-2">
				<span class="mr-2">Verified: </span>
				<ToggleSwitch
					bind:checked={onlyVerified}
					color="var(--verified)"
				/>
			</div>
			<div class="mr-2 border-r border-r-white pr-2">
				<span class="mr-2">Curated: </span>
				<ToggleSwitch
					bind:checked={onlyCurated}
					color="var(--curated)"
				/>
			</div>

			<span>BPM</span>

			<div class="w-48">
				<RangeSlider
					bind:values={bpmValues}
					max={500}
					float
					range
					pushy
				/>
			</div>
		</div>
	</div>
</div>
