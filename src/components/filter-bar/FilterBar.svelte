<script lang="ts">
	import ToggleSwitch from '../common/ToggleSwitch.svelte';
	import RangeSlider from 'svelte-range-slider-pips';
	import Select from 'svelte-select';
	import { DateInput } from 'date-picker-svelte';

	export let searchType = 'Latest';
	export let onlyRanked = false;
	export let onlyVerified = false;
	export let onlyCurated = false;
	export let bpmValues = [0, 500];
	export let startDate = new Date('2018-01-01');
	export let endDate = new Date();

	let selectItems = ['Latest', 'Relevance', 'Rating', 'Curated'];
</script>

<div
	class="my-6 flex w-5/6 select-none flex-col items-center rounded-2xl bg-neutral-900 p-2 2xl:flex-row"
>
	<div
		class=" mr-4 flex w-1/2 items-center pr-4 text-center font-bold 2xl:mb-0 2xl:w-1/6 2xl:border-r 2xl:border-r-white"
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

	<div class="flex w-full items-center 2xl:pl-0">
		<span class="hidden text-2xl font-bold 2xl:flex">Filters</span>

		<div
			class="ml-0 mt-2 flex h-fit w-full items-center rounded-2xl bg-neutral-800 px-0 pl-2 text-xl 2xl:ml-2 2xl:mt-0 2xl:px-2"
		>
			<div
				class="mt-1 flex w-full flex-col items-center 2xl:mt-0 2xl:flex-row"
			>
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
						class="mr-2 flex border-r border-r-transparent pr-2 2xl:border-r-neutral-600"
					>
						<span class="mr-2">Curated: </span>
						<ToggleSwitch
							bind:checked={onlyCurated}
							color="var(--curated)"
						/>
					</div>
				</div>

				<div
					class="2xl-border-r flex w-full min-w-[200px] items-center justify-center 2xl:justify-normal"
				>
					<span>BPM</span>
					<div class="w-full">
						<RangeSlider
							id="slider"
							bind:values={bpmValues}
							max={500}
							float
							range
							pushy
						/>
					</div>
				</div>
				<div
					class="mb-2 flex h-8 w-4/5 items-center pl-0 text-xl 2xl:mb-0 2xl:border-l 2xl:border-l-neutral-600 2xl:pl-2"
				>
					<span class="mr-2 hidden 2xl:inline">Date</span>
					<div class="flex h-2 w-full items-center justify-center">
						<DateInput
							format={'dd-MM-yyyy'}
							placeholder={'start-date'}
							bind:value={startDate}
							min={new Date('2018-01-01')}
							max={endDate}
						/>
						-
						<DateInput
							format={'dd-MM-yyyy'}
							placeholder={'end-date'}
							bind:value={endDate}
							min={startDate}
							max={new Date()}
						/>
					</div>
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
		--border-radius: 8px;
		--placeholder-color: white;
		--height: 48px;
		--item-is-active-bg: var(--secondary);
	}
</style>
