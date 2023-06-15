<script lang="ts">
	import type { Map } from 'src/types/Map';
	import MapCard from './MapCard.svelte';

	export let maps: Map[] = [];
</script>

<div
	class="table-container grid w-5/6 grid-cols-1 gap-x-4 gap-y-4 md:grid-cols-2 xl:grid-cols-3"
>
	{#each maps as map}
		<MapCard {map} />
	{/each}
</div>
