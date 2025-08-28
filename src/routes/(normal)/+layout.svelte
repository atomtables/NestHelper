<script>
	import favicon from '$lib/assets/favicon.png';
	import flow from '$lib/assets/flow.png';
	import files from '$lib/assets/files.png';
	import save from '$lib/assets/save.png';
	import {app, auth, currentFlow, filesystem} from '$lib/state/states.svelte.ts';
	import { page } from "$app/state";
	import image from "$lib/assets/favicon.png";
	import Spinner from "$lib/components/Spinner.svelte";
	import Dropdown from "$lib/components/Dropdown.svelte";
	import Button from "$lib/components/Button.svelte";
	import { slide, fade, draw, scale } from 'svelte/transition';
	import {quintIn} from "svelte/easing";
	import {onMount} from "svelte";
	import {goto} from "$app/navigation";
	import {confirm} from "$lib/components/Dialog.svelte";
	import {getCurrentWindow} from "@tauri-apps/api/window";
	import Input from "$lib/components/Input.svelte";
	import Workflow from "$lib/conn/Workflow.svelte.ts";
	import Flows from "$lib/conn/Flows.ts";
	import command from "$lib/assets/command.png";
	import flows from "$lib/assets/flows.png";

	let { children } = $props();
	let open = $state(false);
	let saveBackup = $state(false);

	// Filesystem to save
	const saveFiles = async () => {
		let filesModified = []
		for (const key in filesystem.value.fileData) {
			let arr1 = filesystem.value.fileData[key].original;
			let arr2 = filesystem.value.fileData[key].modified;
			if (!(arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]))) {
				filesModified.push(key)
			} else if (filesystem.value.fileData[key].newFile || filesystem.value.fileData[key].deletedFile) {
				filesModified.push(key)
			}
		}

		globalThis.filesModified = filesModified;
		globalThis.filesystem = filesystem

		let [result] = await confirm(
				"Are you sure you would like to save all files?",
				`You will be saving all files that you have modified. This includes the following files: <br> ${filesModified.map(f => `<b>${f}</b>`).join("<br>")} <br> Would you like to continue?`,
				saveFilesBackupConfirmation,
				true
		)
		if (!result) return;
		currentFlow.value = new Workflow(Flows.saveFiles(filesModified, saveBackup), "Save modified files to server")
		await currentFlow.value.start();
		await goto("/flow")
		currentFlow.value.promise.then(() => {
			filesystem.value.filesWereModified = false;
			filesystem.value.fileData = {};
			filesystem.value.lastUpdated = new Date(0);
			app.value.status = "Files saved successfully!";
		}).catch((e) => {
			app.value.status = `Error saving files: ${e instanceof Error ? e.message : e}`;
			alert("Error saving files", "There was an error saving your files. Check the flow for any error logs that may have appeared.")
		});
	}
	// Filesystem to check if files modified.
	$effect(() => {
		if (!filesystem.value?.files || !filesystem.value?.fileData) {
			return;
		}
		for (let [key, val] of Object.entries(filesystem.value.fileData)) {
			if (val.original.length !== val.modified.length || val.original.some((v, i) => v !== val.modified[i])) {
				filesystem.value.filesWereModified = true;
				return;
			} else if (val.newFile || val.deletedFile) {
				filesystem.value.filesWereModified = true;
				return;
			}
		}
		filesystem.value.filesWereModified = false;
	})

	onMount(() => {
		getCurrentWindow().onCloseRequested(async (e) => {
			const [confirmed] = await confirm('Are you sure you want to close the app?', 'You may have unsaved changes that will be lost if you close the app.');
			if (!confirmed) {
				e.preventDefault();
			} else {
				await getCurrentWindow().destroy()
			}
		});
	})
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{#snippet saveFilesBackupConfirmation()}
	<div class="flex flex-row items-center gap-2">
		<Input type="checkbox" bind:value={saveBackup} class="w-min" />
		<div class="text-sm">
			Save a backup of all files beforehand.
		</div>
	</div>
{/snippet}

<div class="h-(--top-bar) relative z-50 bg-purple-200 dark:bg-purple-900 flex flex-row justify-between items-center backdrop-blur-sm" data-tauri-drag-region>
	{#if app.value.persistentStoresLoaded && auth.value.username}
		{@const path = page.url.pathname}
		<div class="rounded-2xl flex flex-row items-center h-8 select-none gap-2 z-50">
			<div class="transition-colors hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 rounded-full w-18.5 h-6 px-2 py-1 ml-2"></div>
			<a href="/main" aria-current="{path === '/main'}" class="z-50 group relative transition-colors flex flex-row items-center hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 {path === '/main' && 'border-b-2'} p-2">
				<img src={image} alt="Home" class="w-6 h-4 object-scale-down" />
				<div class="z-50 absolute -bottom-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
					Home
				</div>
			</a>
			<a href="/flows" aria-current="{path === '/flows'}" class="group relative transition-colors flex flex-row items-center hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 {path === '/flows' && 'border-b-2'} p-2">
				<img src={flows} alt="Flows" class="w-6 h-5 object-scale-down" />
				<div class="z-50 absolute -bottom-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
					Flows
				</div>
			</a>
			<a href="/flow" aria-current="{path === '/flow'}" class="group relative transition-colors flex flex-row items-center hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 {path === '/flow' && 'border-b-2'} p-2">
				<img src={flow} alt="Flow" class="w-6 h-4 mt-0.5 object-scale-down" />
				<div class="z-50 text-nowrap absolute -bottom-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
					Active Flow
				</div>
			</a>
			<a href="/files" aria-current="{path === '/files'}" class="group relative transition-colors flex flex-row items-center hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 {path === '/files' && 'border-b-2'} p-2">
				<img src={files} alt="Flow" class="w-6 h-4 object-scale-down" />
				<div class="z-50 absolute -bottom-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
					Files
				</div>
			</a>
			<a href="/command" aria-current="{path === '/command'}" class="group relative transition-colors flex flex-row items-center hover:bg-purple-300 dark:hover:bg-purple-800 active:bg-purple-400 dark:active:bg-purple-700 {path === '/command' && 'border-b-2'} p-2">
				<img src={command} alt="Flow" class="w-6 h-4 object-scale-down" />
				<div class="z-50 absolute -bottom-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
					Commands
				</div>
			</a>
		</div>
		<div class="h-8 flex flex-row items-center justify-center gap-1">
			{#if currentFlow.value?.started}
				<div class="relative inline-block" transition:scale>
					<Button
							transparent
							class="[&]:px-2 grid place-items-center transition-colors w-8 h-8 relative {open && '!bg-neutral-400/50'}"
							onclick={() => open = !open}
					>
						{#if currentFlow.value.complete}
							<div class="absolute top-0 left-0 right-0 bottom-0 w-8 h-8 flex items-center justify-center">
								<div transition:fade={{easing: quintIn}} class=" w-6 h-6 rounded-full bg-purple-500 flex items-center justify-center peer-checked:bg-purple-700 transition-colors">
									<svg
											viewBox="0 0 24 24"
											class="w-4 h-4 text-white scale-100 peer-checked:scale-100"
									>
										<path in:draw
											  fill="none"
											  stroke="currentColor"
											  stroke-width="3"
											  stroke-linecap="round"
											  stroke-linejoin="round"
											  d="M5 13l4 4L19 7"
										/>
									</svg>
								</div>
							</div>
						{:else if currentFlow.value.failed}
							<div class="absolute top-0 left-0 right-0 bottom-0 w-8 h-8 flex items-center justify-center">
								<div transition:fade class="rounded-full w-6 h-6 bg-red-500 flex items-center justify-center peer-checked:bg-red-700 transition-colors">
									<svg
											viewBox="0 0 24 24"
											class="w-4 h-4 text-white scale-100 peer-checked:scale-100"
									>
										<path in:draw
											  fill="none"
											  stroke="currentColor"
											  stroke-width="3"
											  stroke-linecap="round"
											  stroke-linejoin="round"
											  d="M18 6L6 18M6 6l12 12"
										/>
									</svg>
								</div>
							</div>
						{:else}
							<div class="absolute top-0 left-0 right-0 bottom-0 w-8 h-8 flex items-center justify-center">
								<Spinner size="24" type="secondary" />
							</div>
						{/if}
					</Button>

					{#if open}
						<div
								class="absolute z-50 p-5 rounded-2xl top-[calc(100%+0.25rem)] right-0 min-w-max overflow-hidden bg-slate-300 dark:bg-slate-700 shadow-2xl shadow-black"
								transition:slide={{ duration: 150 }}
						>
							<div class="text-sm">
								Running Flow: <b>{currentFlow.value.name}</b>
							</div>
							<div class="text-2xl">
								{#if currentFlow.value.complete}
									Completed
								{:else if currentFlow.value.failed}
									Failed while: <b>{currentFlow.value?.task?.task || "Unknown Task"}</b>
								{:else}
									{currentFlow.value?.task?.task || "In Progress..."}
								{/if}
							</div>
						</div>
					{/if}
				</div>
			{/if}
			{#if filesystem.value.filesWereModified}
				<div class="border-b-2 border-amber-500" transition:scale>
					<Button disableLoading transparent class="[&]:p-1 w-8 h-8 rounded-none group relative" onclick={saveFiles}>
						<img src={save} alt="Flow" class="h-6" />
						<div class="w-max font-normal z-50 absolute -bottom-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
							Save changes to unsaved files
						</div>
					</Button>
				</div>
			{/if}
			<div class="mr-2 select-none text-sm">{auth.value.username}@hackclub.app</div>
		</div>
	{/if}
</div>
<div class="w-screen h-[calc(100vh-var(--both-bars))] overflow-auto overscroll-none relative">
	{@render children?.()}
</div>
<div class="h-(--bottom-bar) z-50 bg-purple-50 dark:bg-purple-950 flex flex-row items-center backdrop-blur-sm" data-tauri-drag-region>
	{#if app.value.status}
		<div class="text-xs mb-0.5 ml-5">
			{app.value.status}
		</div>
	{/if}
	<div class="text-xs mb-0.5 ml-5">
		{#each app.value.pageActions as [name, action]}
			<button class="inline-block px-2 cursor-pointer underline" onclick={async () => await action()}>
				{name}
			</button>
		{/each}
	</div>
</div>

