<script>
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import {onMount} from "svelte";

	// when using `"withGlobalTauri": true`, you may use
	// const { getCurrentWindow } = window.__TAURI__.window;

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="h-9 bg-purple-200 dark:bg-purple-900 flex flex-row justify-between items-center" data-tauri-drag-region>
	<div class="text-sm opacity-0">NestHelper</div>
	<div class="text-sm absolute top-2 right-1/2 translate-x-1/2">NestHelper</div>
	<div class="text-sm mr-2">home|etc</div>
</div>
{@render children?.()}
