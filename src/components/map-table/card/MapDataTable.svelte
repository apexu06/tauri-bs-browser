<script lang="ts">
	import type { MapDetail } from 'src/types/MapDetail';
	import ScrollText from 'src/components/common/ScrollText.svelte';
	export let map: MapDetail;

	let mapperDisplay: HTMLDivElement;
	let mapperDisplayWidth: number;
	let mapperLabel: HTMLElement;
	let mapperLabelWidth: number;
	let mapperSpan: HTMLSpanElement;
	let mapperSpanWidth: number;
	let songAuthorContainer: HTMLDivElement;
	let songAuthorContainerWidth: number;

	window.addEventListener('resize', () => {
		mapperDisplayWidth = mapperDisplay?.clientWidth;
		mapperLabelWidth = mapperLabel?.clientWidth;
		mapperSpanWidth = mapperSpan?.clientWidth;
		songAuthorContainerWidth = songAuthorContainer?.clientWidth;
	});

	$: mapperDisplayWidth = mapperDisplay?.clientWidth;
	$: mapperLabelWidth = mapperLabel?.clientWidth;
	$: mapperSpanWidth = mapperSpan?.clientWidth;
	$: songAuthorContainerWidth = songAuthorContainer?.clientWidth;
</script>

<div class="z-10 flex flex-col">
	<div class="overflow-hidden" bind:this={songAuthorContainer}>
		<ScrollText containerWidth={songAuthorContainerWidth} style="text-xl"
			>{map.metadata.songAuthorName}</ScrollText
		>
	</div>
	<div class="mb-1 h-px w-full bg-gray-50" />

	<div
		class="relative flex w-full justify-between overflow-hidden"
		bind:this={mapperDisplay}
	>
		<b bind:this={mapperLabel}>Mapper:</b>
		<span
			class="flex translate-x-0 justify-between whitespace-nowrap transition duration-1000 {mapperLabelWidth +
				mapperSpanWidth >
			mapperDisplayWidth
				? 'scrollText'
				: ''}"
			style="--spanWidth: {mapperDisplayWidth - mapperLabelWidth}px"
			bind:this={mapperSpan}
		>
			{map.metadata.levelAuthorName}</span
		>
	</div>
	<span class="flex justify-between"
		><b>Published:</b> {map.lastPublishedAt.split('T')[0]}</span
	>
	<span class="flex justify-between"
		><b>BPM:</b> {Math.round(map.metadata.bpm)}</span
	>
	<span class="flex justify-between"
		><b>Duration:</b>
		{Math.floor(map.metadata.duration / 60)}:{map.metadata.duration %
			60}</span
	>
</div>

<style>
	.scrollText {
		border-radius: 8px;
		padding-left: 2px;
		padding-right: 2px;
	}
	.scrollText:hover {
		transform: translateX(calc(var(--spanWidth) - 100%));
		background: #444444;
	}
</style>
