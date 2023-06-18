<script lang="ts">
	export let checked = false;
	export let color: string;
</script>

<label class="switch">
	<input bind:checked type="checkbox" />
	<span class="slider round" style="--color: {color}" />
</label>

<style>
	.switch {
		position: relative;
		display: inline-block;
		width: 51px;
		height: 25px;
	}

	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: #555;
		-webkit-transition: 0.2s;
		transition: 0.2s;
	}

	.slider:before {
		position: absolute;
		content: '';
		height: 17px;
		width: 17px;
		left: 4px;
		bottom: 4px;
		background-color: white;
		-webkit-transition: 0.2s;
		transition: 0.2s;
	}

	input:checked + .slider {
		background-color: var(--color);
	}

	input:focus + .slider {
		box-shadow: 0 0 1px var(--color);
	}

	input:checked + .slider:before {
		-webkit-transform: translateX(26px);
		-ms-transform: translateX(26px);
		transform: translateX(26px);
	}

	/* Rounded sliders */
	.slider.round {
		border-radius: 34px;
	}

	.slider.round:before {
		border-radius: 50%;
	}
</style>
