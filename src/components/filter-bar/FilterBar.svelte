<script lang="ts">
	import ToggleSwitch from '../common/ToggleSwitch.svelte';
	import RangeSlider from 'svelte-range-slider-pips';
	import Select from 'svelte-select';

	export let searchType = 'Latest';
	export let onlyRanked = false;
	export let onlyVerified = false;
	export let onlyCurated = false;

	export let bpmValues = [0, 500];

	let selectItems = ['Latest', 'Relevance', 'Rating', 'Curated'];
</script>

<div
	class="my-6 flex w-5/6 flex-col items-center rounded-lg bg-neutral-900 p-2 lg:flex-row"
>
	<div
		class="mb-2 mr-4 flex w-1/2 items-center pl-2 pr-4 text-center font-bold lg:mb-0 lg:w-1/6 lg:border-r lg:border-r-white"
	>
		<Select
			items={selectItems}
			closeListOnChange
			on:change={(e) => (searchType = e.detail.value)}
			class="searchSelect"
			clearable={false}
			searchable={false}
			value={'Latest'}
		/>
	</div>

	<div class="mr-4 flex w-full items-center pl-2 lg:pl-0">
		<span class="hidden text-xl font-bold lg:flex">Filters</span>

		<div
			class="ml-2 flex h-fit w-full items-center rounded-lg bg-neutral-800 px-2 py-1 text-lg"
		>
			<div class=" flex w-full flex-col items-center lg:flex-row">
				<div class="flex">
					<div class="mr-2 flex border-r border-r-neutral-600 pr-2">
						<span class="mr-2">Ranked: </span>
						<ToggleSwitch
							bind:checked={onlyRanked}
							color="var(--ranked)"
						/>
					</div>
					<div class="mr-2 flex border-r border-r-neutral-600 pr-2">
						<span class="mr-2">Verified: </span>
						<ToggleSwitch
							bind:checked={onlyVerified}
							color="var(--verified)"
						/>
					</div>
					<div
						class="mr-2 flex border-r border-r-transparent pr-2 lg:border-r-white"
					>
						<span class="mr-2">Curated: </span>
						<ToggleSwitch
							bind:checked={onlyCurated}
							color="var(--curated)"
						/>
					</div>
				</div>

				<div
					class="lg-border-r flex w-full items-center justify-center lg:justify-normal"
				>
					<span>BPM</span>
					<div class="w-full lg:w-1/2 lg:border-r lg:border-r-white">
						<RangeSlider
							id="slider"
							bind:values={bpmValues}
							max={500}
							float
							range
							pushy
						/>
					</div>

					<span>Date</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	:global(.searchSelect) {
		background: linear-gradient(
			to right,
			var(--primary),
			var(--secondary)
		) !important;
		border: none !important;
		--item-color: var(--primary);
		--list-background: var(--bgColor);
		--item-hover-bg: #555;
		--input-color: white;
		--placeholder-color: white;
		--value-container-padding: -10px;
		--item-is-active-bg: var(--secondary);
	}
</style>
