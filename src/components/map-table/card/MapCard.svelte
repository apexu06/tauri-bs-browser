<script lang="ts">
	import type { MapDetail } from 'src/types/MapDetail';
	import pauseButton from '../../../icons/pause-button.svg';
	import playButton from '../../../icons/play-button.svg';
	import { onMount } from 'svelte';
	import MapDataTable from './MapDataTable.svelte';

	let ratingBar: HTMLDivElement;
	let ratingBarWidth: number;

	export let map: MapDetail;
	let diffs = map.versions[0].diffs;
	let likes = map.stats.upvotes;
	let dislikes = map.stats.downvotes;

	let songTitleSpan: HTMLSpanElement;
	let songTitleContainer: HTMLDivElement;
	let songTitleSpanWidth: number;
	let songTitleContainerWidth: number;

	onMount(() => {
		getMapColors();
	});

	window.addEventListener('resize', () => {
		ratingBarWidth = ratingBar?.clientWidth;
		songTitleSpanWidth = songTitleSpan?.clientWidth;
		songTitleContainerWidth = songTitleContainer?.clientWidth;
	});

	function getDiffColor(diff: string) {
		switch (diff) {
			case 'Easy':
				return 'bg-green-500';
			case 'Normal':
				return 'bg-blue-500';
			case 'Hard':
				return 'bg-yellow-500';
			case 'Expert':
				return 'bg-red-500';
			case 'ExpertPlus':
				return 'bg-pink-500';
			default:
				return 'bg-gray-500';
		}
	}

	let color1 = '';
	let color2 = '';

	function getMapColors() {
		if (map.ranked) {
			color1 = '#16f768';
		}
		color2 = color1;

		if (map.curatedAt !== null) {
			color2 = '#f49004';
		} else if (
			map.uploader.verifiedMapper !== null &&
			map.uploader.verifiedMapper
		) {
			color2 = '#ab04f2';
		}

		if (color2 !== '' && color1 === '') {
			color1 = color2;
		}

		color1.replace(/(['"])/g, '');
		color2.replace(/(['"])/g, '');
	}

	$: ratingBarWidth = ratingBar?.clientWidth;
	$: songTitleSpanWidth = songTitleSpan?.clientWidth;
	$: songTitleContainerWidth = songTitleContainer?.clientWidth;

	$: console.log(songTitleContainerWidth + ' ' + songTitleSpanWidth);
</script>

<div
	class="image-container flex h-56 w-full flex-col rounded-xl bg-neutral-700 transition"
	style="--image: url({map.versions[0]
		.coverURL}); --color1: {color1}; --color2: {color2}"
>
	<div
		class="flex h-full w-full items-center justify-center rounded-xl backdrop-blur-[3px]"
	>
		<div class="content-container grid h-full w-full grid-cols-2 p-4">
			<div class="col-span-2 flex flex-col">
				<div
					class="relative w-full overflow-hidden"
					bind:this={songTitleContainer}
				>
					<h4 class="col-span-2 font-bold leading-7">
						<span
							class="inline-block translate-x-0 whitespace-nowrap transition duration-1000 {songTitleSpanWidth >
							songTitleContainerWidth
								? 'scrollText'
								: ''}"
							style="--headingWidth: {songTitleSpanWidth}px; --containerWidth: {songTitleContainerWidth}px;"
							bind:this={songTitleSpan}
						>
							{map.name}</span
						>
					</h4>
				</div>
				<h5 class="mb-1 italic leading-4">
					{map.metadata.songSubName}
				</h5>
			</div>

			<MapDataTable {map} />

			<div
				class="ml-2 flex h-full w-full flex-col items-center justify-center"
			>
				<div
					class=" flex w-fit max-w-[80%] justify-between rounded-lg bg-transparent text-[80%] font-bold backdrop-blur-md"
				>
					{#each diffs as diff}
						<div
							class="{getDiffColor(
								diff.difficulty
							)} m-1 h-4 w-12 items-center justify-center rounded-xl text-center"
						>
							{#if diff.difficulty == 'ExpertPlus'}
								<span>E+</span>
							{:else}
								<span>{diff.difficulty.charAt(0)}</span>
							{/if}
						</div>
					{/each}
				</div>

				<div
					class="mt-2 flex h-6 w-4/5 overflow-hidden rounded-lg bg-transparent text-center font-bold backdrop-blur-xl"
					bind:this={ratingBar}
				>
					{#if likes == 0 && dislikes == 0}
						<div
							class="w-full rounded-lg bg-transparent backdrop-blur-xl"
						>
							No Rating yet
						</div>
					{:else}
						<div
							class="rounded-l-lg bg-green-400"
							style="width: {(likes / (likes + dislikes)) *
								ratingBarWidth}px;"
						>
							{likes}
						</div>
						<div
							class="min-w-[10%] rounded-r-lg bg-red-400"
							style="width: {(dislikes / (likes + dislikes)) *
								ratingBarWidth}px;"
						>
							{dislikes}
						</div>
					{/if}
				</div>
				<button
					class="mt-4 flex h-12 w-4/5 items-center justify-center rounded-xl bg-transparent hover:bg-transparent"
					><img
						src={playButton}
						width="30"
						height="30"
						alt="Play"
					/></button
				>
			</div>
		</div>
	</div>
</div>

<style>
	.image-container {
		background: rgba(0, 0, 0, 0.5) var(--image);
		background-repeat: no-repeat;
		background-size: cover;
		background-position-y: 50%;
		background-blend-mode: darken;
		border: 2px solid var(--bgColor);
		transition: 200ms;
		position: relative;
	}

	.image-container:hover {
		background: rgba(0, 0, 0, 0.7) var(--image);
		background-repeat: no-repeat;
		background-size: cover;
		background-position-y: 30%;
		background-blend-mode: darken;
		transition: 200ms;
		border: 2px solid white;
		cursor: pointer;
	}

	.image-container::before {
		content: '';
		position: absolute;
		z-index: -1;
		inset: -1px;
		background: linear-gradient(
			-90deg,
			var(--color1) 0%,
			var(--color2) 80%
		);
		transform: translate(7px, 7px);
		filter: blur(5px);
		opacity: 1;
		transition: opacity 0.3s;
		border-radius: inherit;
	}

	button {
		backdrop-filter: blur(15px);
	}

	button:hover {
		border: none;
		transform: scaleY(120%);
	}

	.scrollText {
		border-radius: 8px;
		padding-left: 2px;
		padding-right: 2px;
	}
	.scrollText:hover {
		transform: translateX(
			calc((var(--containerWidth) - var(--headingWidth)) - 10px)
		);
		background: #444444;
	}
</style>