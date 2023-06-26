<script lang="ts">
	import type { MapDetail } from 'src/types/MapDetail';
	import MapCard from './card/MapCard.svelte';

	let card: HTMLDivElement;
	export let maps: MapDetail[] = [];
	export let getMaps: (
		currentMaps: MapDetail[],
		page: number
	) => Promise<void>;

	let page = 0;

	function test() {
		let observer = new IntersectionObserver(
			async () => {
				await getMaps(maps, page++);
			},
			{ threshold: 1 }
		);
		observer.observe(card);
	}

	$: card && test();
</script>

<div
	class="table-container grid w-5/6 grid-cols-1 gap-x-4 gap-y-4 md:grid-cols-2 xl:grid-cols-3"
>
	{#each maps as map, index}
		{#if index === maps.length - 1}
			<div bind:this={card}>
				<MapCard {map} />
			</div>
		{/if}
		<MapCard {map} />
	{/each}
</div>
