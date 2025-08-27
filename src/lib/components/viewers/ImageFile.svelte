<script>
    import Editor from "$lib/components/Editor.svelte";
    import Dropdown from "$lib/components/Dropdown.svelte";
    import {highlighting, supportedLanguages} from "$lib/helpers/monaco.ts";
    import {onMount} from "svelte";
    import {alert} from "$lib/components/Dialog.svelte";
    import {filesystem} from "$lib/state/states.svelte.ts";
    import { event } from '@tauri-apps/api';
    import { confirm } from "$lib/components/Dialog.svelte";
    import saveIcon from "$lib/assets/save.png";
    import back from "$lib/assets/back.png";
    import {save} from "@tauri-apps/plugin-dialog";
    import {create} from "@tauri-apps/plugin-fs";

    let {
        filePath,
        filename,
        nvm
    } = $props()

    let file = $state()
    onMount(() => {
        try {
            // noinspection JSUnresolvedReference lazy ass code writing and intellij sucks for js types
            file = `data:image/unknown;base64,${filesystem.value.fileData[filePath].modified.toBase64()}`
        } catch (e) {
            console.error("Error decoding file:", e);
            alert("Error", `There was an error decoding the file: ${e.message}. Ensure you selected the correct file type.`);
            file = null;
        }
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
</script>

{#if file != null}
    <div class="flex flex-col w-full h-full max-h-full gap-2">
        <div class="flex flex-row items-center justify-between flex-nowrap gap-2">
            <button onclick={() => nvm()} class="group relative cursor-pointer text-sm text-neutral-300 shrink-0 h-6 w-6 flex items-center justify-center rounded-full bg-neutral-700 hover:bg-neutral-600 active:bg-neutral-500 transition-colors">
                <img src={back} alt="back" class="h-5">
                <span class="z-50 absolute -bottom-8 py-1 px-2 left-0 text-nowrap bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                    Back
                </span>
            </button>
            <div class="text-sm text-neutral-300 grow">{filePath}</div>
            <button onclick={() => saveFile()} class="group relative cursor-pointer text-sm text-neutral-300 shrink-0 h-6 w-6 flex items-center justify-center rounded-full bg-neutral-700 hover:bg-neutral-600 active:bg-neutral-500 transition-colors">
                <img src={saveIcon} alt="save" class="h-5">
                <span class="z-50 absolute -bottom-8 py-1 px-2 left-1/2 text-nowrap -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                    Save file
                </span>
            </button>
            <div>(read only)</div>
        </div>
        <div class="flex flex-col max-w-full min-h-0">
            <img src={file} alt={filePath} class="object-scale-down" />
        </div>
    </div>
{/if}