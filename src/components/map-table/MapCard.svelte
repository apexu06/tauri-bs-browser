<script lang="ts">
	import type { Map } from 'src/types/Map';
	import '../../styles.css';
	import pauseButton from '../../icons/pause-button.svg';
	import playButton from '../../icons/play-button.svg';

	export let map: Map;
</script>

<div
	class="image-container flex h-56 w-full flex-col rounded-xl bg-neutral-700 transition"
	style="--image: url({map.versions[0].coverURL})"
>
	<div
		class="flex h-full w-full items-center justify-center rounded-xl backdrop-blur-[3px]"
	>
		<div class="content-container grid h-full w-full grid-cols-2 py-4 pl-4">
			<div class="col-span-2 flex flex-col">
				<h4 class="col-span-2 overflow-hidden font-bold leading-7">
					{map.metadata.songName}
				</h4>
				<h5 class="mb-1 italic leading-3">
					{map.metadata.songSubName}
				</h5>
			</div>

			<div class="flex flex-col">
				<span class="text-xl">{map.metadata.songAuthorName}</span>
				<div class="mb-1 h-px w-full bg-gray-50" />

				<span class="flex justify-between"
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

			<div class="flex h-full w-full items-center justify-center">
				<button
					class="flex h-24 w-24 items-center justify-center rounded-xl bg-transparent"
					><img
						src={playButton}
						width="50"
						height="50"
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
		z-index: 100;
	}

	span {
		word-wrap: break-word;
	}

	button {
		backdrop-filter: blur(15px);
	}

	button:hover {
		border: none;
	}
</style>
