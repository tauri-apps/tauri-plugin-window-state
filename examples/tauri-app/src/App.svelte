<script>
  import Greet from './lib/Greet.svelte'
	import { getCurrent } from '@tauri-apps/api/window'

	const webview = getCurrent()
	const handleFullscreen = async () => {
		await webview.setFullscreen(!(await webview.isFullscreen()))
	}

	const handleClose = async () => {
		await webview.close()
	}
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank noreferrer">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank noreferrer">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank noreferrer">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div>
    <button on:click={handleFullscreen}>Swtich Fullscreen</button>
    <button on:click={handleClose}>Close</button>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
