<script lang="ts">
	export let containerWidth: number;
	export let style: string;

	let textSpan: HTMLSpanElement;
	let textSpanWidth: number;

	window.addEventListener('resize', () => {
		textSpanWidth = textSpan?.clientWidth;
	});

	$: textSpanWidth = textSpan?.clientWidth;
</script>

<span
	class="inline-block translate-x-0 whitespace-nowrap transition duration-1000 {style} {textSpanWidth >
	containerWidth
		? 'scrollText'
		: ''}"
	style="--headingWidth: {textSpanWidth}px; --containerWidth: {containerWidth}px;"
	bind:this={textSpan}><slot /></span
>

<style>
	.scrollText {
		border-radius: 8px;
		padding-left: 2px;
	}
	.scrollText:hover {
		transform: translateX(
			calc((var(--containerWidth) - var(--headingWidth)) - 5px)
		);
	}
</style>
