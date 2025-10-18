<script>
    import Editor from '$lib/components/Editor.svelte';
    import Dropdown from '$lib/components/generic/Dropdown.svelte';
    import { highlighting, supportedLanguages } from '$lib/helpers/monaco.ts';
    import { onMount } from 'svelte';
    import { alert } from '$lib/components/generic/Dialog.svelte';
    import { filesystem } from '$lib/state/states.svelte.ts';
    import { event } from '@tauri-apps/api';
    import { confirm } from '$lib/components/generic/Dialog.svelte';
    import saveIcon from '$lib/assets/save.png';
    import back from '$lib/assets/back.png';
    import { save } from '@tauri-apps/plugin-dialog';
    import { create } from '@tauri-apps/plugin-fs';

    let { filePath, filename, nvm } = $props();

    let file = $derived(filesystem.value.fileData[filePath].modified);
</script>

{#if file != null}
    <div class="flex flex-col w-full h-full max-h-full gap-2">
        <div class="flex flex-row items-center justify-between flex-nowrap gap-2">
            <button
                onclick={() => nvm()}
                class="group relative cursor-pointer text-sm text-neutral-300 shrink-0 h-6 w-6 flex items-center justify-center rounded-full bg-neutral-700 hover:bg-neutral-600 active:bg-neutral-500 transition-colors"
            >
                <img src={back} alt="back" class="h-5" />
                <span class="z-50 absolute -bottom-8 py-1 px-2 left-0 text-nowrap bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all"> Back </span>
            </button>
            <div class="text-sm text-neutral-300 grow">{filePath}</div>
            <button
                onclick={() => saveFile()}
                class="group relative cursor-pointer text-sm text-neutral-300 shrink-0 h-6 w-6 flex items-center justify-center rounded-full bg-neutral-700 hover:bg-neutral-600 active:bg-neutral-500 transition-colors"
            >
                <img src={saveIcon} alt="save" class="h-5" />
                <span class="z-50 absolute -bottom-8 py-1 px-2 left-1/2 text-nowrap -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                    Save file
                </span>
            </button>
            <div>(read only)</div>
        </div>
        <div class="flex flex-row max-w-full min-h-0 justify-between">
            <div class="overflow-auto flex flex-row h-full">
                <div class="pr-2 h-full">
                    <div class="flex flex-row flex-wrap gap-2 w-124">
                        {#each file as byte, i}
                            {@const value = (() => {
                                let val = byte.toString(16).split('');
                                if (val.length === 0) return ['0', '0'];
                                else if (val.length === 1) return ['0', ...val];
                                else return val;
                            })()}
                            <div class="p-0.5 flex flex-row {i % 2 === 0 ? 'text-neutral-300' : 'text-neutral-400'} bg-neutral-800/50 hover:bg-neutral-700 cursor-default">
                                <div>
                                    {value[0]}
                                </div>
                                <div>
                                    {value[1]}
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
                <div class="flex flex-row flex-wrap gap-1 w-60 pl-2 h-full">
                    {#each file as byte, i}
                        {@const value = (() => {
                            if (byte < 32 || byte >= 127) return null;
                            console.log(byte, String.fromCharCode(byte));
                            return String.fromCharCode(byte);
                        })()}
                        <div class="font-mono py-0.5 pb-1.5 text-neutral-500 flex flex-row cursor-default">
                            <div class="whitespace-pre {value === ' ' && 'bg-neutral-700/15'}">{value || '.'}</div>
                        </div>
                    {/each}
                </div>
            </div>

            <div class="">file info</div>
        </div>
    </div>
{/if}
