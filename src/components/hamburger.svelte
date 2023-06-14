<script lang="ts">
	import '../styles.css';
	export let transformed = false;

	let l1 = 'default',
		l2 = 'default',
		l3 = 'default';

	function onClick(transformed: boolean) {
		if (!transformed) {
			l1 = 'default';
			l2 = 'default';
			l3 = 'default';
		} else {
			l1 = 'line1';
			l2 = 'line2';
			l3 = 'line3';
		}
	}

	$: onClick(transformed);
</script>

<div
	class="flex w-12 flex-col"
	on:click={() => {
		transformed = !transformed;
	}}
>
	<div class="m-1 h-2 w-full rounded bg-white {l1}" />
	<div class="m-1 h-2 w-full rounded bg-white {l2}" />
	<div class="m-1 h-2 w-full rounded bg-white {l3}" />
</div>

<style>
	.default {
		transition: 200ms;
	}

	.line1 {
		transition: 200ms;
		transform: translateX(-10px);
	}

	.line2 {
		transition: 200ms;
		transform: translateX(20px);
	}

	.line3 {
		transition: 200ms;
		transform: translateX(-20px);
	}
</style>
