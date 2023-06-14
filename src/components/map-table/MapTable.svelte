<script lang="ts">
	import type { Map } from 'src/types/Map';
	import MapCard from './MapCard.svelte';

	export let maps: Map[] = [];
</script>

<div>
	<MapCard map={maps[0]} />
</div>
