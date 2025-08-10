<script>
	import '../app.css';
	import favicon from '$lib/assets/favicon.png';
	import {appReady, auth} from '$lib/state/states.svelte.ts';
	import image from "$lib/assets/favicon.png";

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="h-(--top-bar) z-50 bg-purple-200 dark:bg-purple-900 flex flex-row justify-between items-center backdrop-blur-sm" data-tauri-drag-region>
	{#if /*appReady.is && auth.set*/ true}
		<div class="rounded-2xl flex flex-row items-center h-8 select-none">
			<div class="transition-colors hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 rounded-full w-18.5 h-6 px-2 py-1 ml-2"></div>
			<button class="transition-colors flex flex-row items-center hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 rounded-full p-2">
				<img src={image} alt="Relaunch the app" class="h-4 w-4" />
			</button>
		</div>
		<div class="mr-2 select-none text-sm">hi@hackclub.app</div>
	{/if}
</div>
<div class="w-screen h-[calc(100vh-var(--top-bar))] overflow-hidden relative">
	{@render children?.()}
</div>
