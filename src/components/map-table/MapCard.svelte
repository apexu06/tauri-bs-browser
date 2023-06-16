<script lang="ts">
	import type { Map } from 'src/types/Map';
	import '../../styles.css';
	import pauseButton from '../../icons/pause-button.svg';
	import playButton from '../../icons/play-button.svg';
	import { onMount } from 'svelte';

	export let map: Map;
	let diffs = map.versions[0].diffs;
	let ratingBar: HTMLDivElement;
	let ratingBarWidth: number;
	let likes = map.stats.upvotes;
	let dislikes = map.stats.downvotes;

	onMount(() => {
		getMapState();
	});

	window.addEventListener('resize', () => {
		ratingBarWidth = ratingBar?.clientWidth;
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

	function getMapState() {
		if (map.ranked) {
			color1 = '#16f768';
		} else if (map.qualified) {
			color1 = '#ab04f2';
		}
		color2 = color1;

		if (map.curatedAt !== null) {
			color2 = '#f49004';
		} else if (
			map.uploader.verifiedMapper !== null &&
			map.uploader.verifiedMapper
		) {
			color2 = '#1b68f7';
		}

		if (color2 !== '' && color1 === '') {
			color1 = color2;
		}

		color1.replace(/(['"])/g, '');
		color2.replace(/(['"])/g, '');
	}

	$: ratingBarWidth = ratingBar?.clientWidth;
</script>

<div
	class="image-container flex h-56 w-full flex-col rounded-xl bg-neutral-700 transition"
	style="--image: url({map.versions[0]
		.coverURL}); --color1: {color1}; --color2: {color2}"
>
	<div
		class="flex h-full w-full items-center justify-center rounded-xl backdrop-blur-[3px]"
	>
		<div class="content-container grid h-full w-full grid-cols-2 py-4 pl-4">
			<div class="col-span-2 flex flex-col">
				<h4
					class="col-span-2 overflow-hidden whitespace-nowrap font-bold leading-7"
				>
					{map.metadata.songName}
				</h4>
				<h5 class="mb-1 italic leading-3">
					{map.metadata.songSubName}
				</h5>
			</div>

			<div class="flex flex-col">
				<span class="text-xl">{map.metadata.songAuthorName}</span>
				<div class="mb-1 h-px w-full bg-gray-50" />

				<span class="flex justify-between whitespace-nowrap"
					><b>Mapper:</b> {map.metadata.levelAuthorName}</span
				>
				<span class="flex justify-between"
					><b>Published:</b> {map.lastPublishedAt.split('T')[0]}</span
				>
				<span class="flex justify-between"
					><b>BPM:</b> {map.metadata.bpm}</span
				>
				<span class="flex justify-between"
					><b>Duration:</b>
					{Math.floor(map.metadata.duration / 60)}:{map.metadata
						.duration % 60}</span
				>
			</div>

			<div
				class="flex h-full w-full flex-col items-center justify-center"
			>
				<div
					class=" flex w-fit max-w-[80%] justify-between rounded bg-transparent text-[80%] font-bold backdrop-blur-md"
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

	span {
		word-wrap: break-word;
	}

	button {
		backdrop-filter: blur(15px);
	}

	button:hover {
		border: none;
		transform: scaleY(120%);
	}
</style>
