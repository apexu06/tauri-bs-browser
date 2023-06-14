<script lang="ts">
	import type { Map } from 'src/types/Map';

	export let map: Map;
</script>

<div class="w-1/3 rounded-xl bg-neutral-500">
	<span>{map.name}</span>
</div>
