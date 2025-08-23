<script>
    import Editor from "$lib/components/Editor.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import {highlighting, supportedLanguages} from "$lib/helpers/monaco.ts";
    import {onMount} from "svelte";
    import {alert} from "$lib/components/Dialog.svelte";
    import {filesystem} from "$lib/state/states.svelte.ts";
    import { save } from '@tauri-apps/plugin-dialog';
    import saveIcon from '$lib/assets/save.png';
    import { event } from '@tauri-apps/api';
    import { confirm } from "$lib/components/Dialog.svelte";
    import Button from "$lib/components/Button.svelte";
    import {BaseDirectory, create} from "@tauri-apps/plugin-fs";

    let {
        filename,
        filePath,
        nvm
    } = $props()

    let file = $state()
    onMount(() => {
        try {
            file = new TextDecoder('utf-8', { fatal: true }).decode(filesystem.value.fileData[filePath].modified)
        } catch (e) {
            console.error("Error decoding file:", e);
            alert("Error", `There was an error decoding the file: ${e.message}. Ensure you selected the correct file type.`);
            file = null;
            nvm?.()
        }
    })

    // file => .modified (changes made by user)
    $effect(() => {
        filesystem.value.fileData[filePath].modified = new TextEncoder('utf-8', { fatal: true }).encode(file)
    })

    const saveFile = async () => {
        const path = await save({
            defaultPath: filename,
            canCreateDirectories: true
        });

        try {
            const file = await create(path);
            await file.write(filesystem.value.fileData[filePath].modified);
            await file.close();
        } catch (e) {
            console.error("Error saving file:", e);
            await alert("Error", `There was an error saving the file: ${e.message}.`);
        }
    }

    let currentLanguage = $state(highlighting(filename));
</script>

{#if file != null}
    <div class="flex flex-col w-full h-full gap-2">
        <div class="flex flex-row items-center justify-between flex-nowrap gap-2">
            <div class="text-sm text-neutral-300 grow">{filePath}</div>
            <button onclick={() => saveFile()} class="group relative cursor-pointer text-sm text-neutral-300 shrink-0 h-6 w-6 flex items-center justify-center rounded-full bg-neutral-700 hover:bg-neutral-600 active:bg-neutral-500 transition-colors">
                <img src={saveIcon} alt="save" class="h-5">
                <span class="z-50 absolute -bottom-8 py-1 px-2 left-1/2 text-nowrap -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                    Save file
                </span>
            </button>
            <Dropdown direction="right" class="[&]:p-0 [background-color:unset]! hover:underline text-sm font-light shrink-0" items={supportedLanguages()}>{currentLanguage}</Dropdown>
        </div>
        <Editor bind:value={file} language={currentLanguage}/>
    </div>
{/if}