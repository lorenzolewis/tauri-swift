<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	var result = "Click the button!"

	function clickHandler() {
		invoke('tauri_get_mounts').then((value: string) => {
			result = value
			console.log(value)
		}).catch((e) => console.error(e))
	}
</script>

<main>
	<h1>Hey you!</h1>
	<p>{result}</p>
	<button on:click={clickHandler}>Get volume</button>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>