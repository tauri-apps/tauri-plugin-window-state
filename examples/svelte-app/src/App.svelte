<script lang="ts">

	import { getCurrent } from '@tauri-apps/api/window'

	// import { getCurrent } from 
	export let name: string;

	const webview = getCurrent()

	const handleFullscreen = async () => {
		await webview.setFullscreen(!(await webview.isFullscreen()))
	}
	
	const handleClose = async () => {
		await webview.close()
	}
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
	<button on:click={handleFullscreen}>Swtich Fullscreen</button>
	<button on:click={handleClose}>Close</button>
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