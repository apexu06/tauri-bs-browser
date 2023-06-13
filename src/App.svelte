<script lang="ts">
	import Greet from './lib/Greet.svelte';
	import './styles.css';
	import Hamburger from './components/hamburger.svelte';
</script>

<main class="flex h-full w-full flex-col items-center justify-center">
	<div class="m-8 h-fit w-fit self-start">
		<Hamburger />
	</div>

	<div class="flex flex-col items-center">
		<h1>BeatSaber Browser</h1>
		<h3>Search Maps</h3>
	</div>

	<div class="flex h-12 w-2/5 shadow shadow-black">
		<input type="text" placeholder="map title" />
		<button>Search</button>
	</div>
</main>

<style>
</style>
