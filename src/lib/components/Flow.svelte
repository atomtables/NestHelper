<script lang="ts">
    import Spinner from '$lib/components/generic/Spinner.svelte';
    import { draw, fade, scale, slide } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { cubicOut, quartInOut, quintIn, quintInOut } from 'svelte/easing';
    import Button from '$lib/components/generic/Button.svelte';
    import { emit } from '@tauri-apps/api/event';
    import { currentFlow } from '$lib/state/states.svelte.js';

    let popShows = $state({});
    function hideAll(i) {
        let val = popShows[i];
        popShows = {};
        popShows[i] = val;
    }
</script>

<div class="flex flex-col items-start justify-center gap-4 p-2 pr-4 w-full">
    {#if currentFlow.value?.tasks}
        {#each currentFlow.value.tasks as task, i}
            <div class="relative flex flex-row items-center justify-between gap-4 w-full dark:bg-purple-800 bg-purple-300 p-2 rounded-full" transition:slide>
                <div class="flex flex-row items-center gap-4">
                    <div class="*:absolute w-8 h-8 pl-1">
                        {#if task.state === 'ongoing'}
                            <div transition:fade={{ easing: quintIn }} class="w-8 h-8 rounded-full flex items-center justify-center transition-colors">
                                <Spinner type="secondary" />
                            </div>
                        {:else if task.state === 'done'}
                            <div transition:fade={{ easing: quintIn }} class="w-8 h-8 rounded-full bg-purple-500 flex items-center justify-center peer-checked:bg-purple-700 transition-colors">
                                <svg viewBox="0 0 24 24" class="w-5 h-5 text-white scale-100 peer-checked:scale-100">
                                    <path in:draw fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
                                </svg>
                            </div>
                        {:else if task.state === 'failed'}
                            <div transition:fade class="w-8 h-8 rounded-full bg-red-500 flex items-center justify-center peer-checked:bg-red-700 transition-colors">
                                <svg viewBox="0 0 24 24" class="w-5 h-5 text-white scale-100 peer-checked:scale-100">
                                    <path in:draw fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" d="M18 6L6 18M6 6l12 12" />
                                </svg>
                            </div>
                        {:else}
                            <div transition:fade={{ easing: quintIn }} class="w-8 h-8 rounded-full border-3 border-blue-500 dark:border-blue-500 animate-pulse flex items-center justify-center transition-colors"></div>
                        {/if}
                    </div>
                    <div class="flex flex-col">
                        <div>{task.task}</div>
                        <div class="text-xs text-neutral-300">{task.description || ''}</div>
                    </div>
                </div>
                <Button class="p-2 rounded-full" onclick={() => ((popShows[i] = !popShows[i]), hideAll(i))}>Output</Button>
                {#if window.matchMedia('(min-width: 40rem)').matches && popShows[i]}
                    <div class="absolute flex flex-col overflow-scroll scroll-auto bg-neutral-200 dark:bg-neutral-900 right-28 h-48 w-96 lg:w-4/5 p-5 z-50 shadow-2xl origin-right" transition:scale={{ duration: 200, easing: quartInOut }}>
                        {#each task.outputArr as output}
                            {#if output.file === 'stdout'}
                                <pre>{output.output}</pre>
                            {:else if output.file === "stderr"}
                                <pre class="text-red-500">{output.output}</pre>
                            {:else}
                                {@html output.output}
                            {/if}
                        {/each}
                    </div>
                {/if}
            </div>
        {/each}
    {:else}
        <div class="flex flex-row items-center justify-center gap-4 w-full dark:bg-purple-800 bg-purple-300 p-2 rounded-full" transition:slide>
            <Spinner type="secondary" />
            <div class="text-lg">Waiting for backend...</div>
        </div>
    {/if}
</div>

{#if Object.entries(popShows).find(([k, v]) => v) && !window.matchMedia('(min-width: 40rem)').matches}
    <div class="absolute p-8 top-0 right-0 left-0 bottom-0" onclick={() => (popShows = {})} role="dialog" tabindex="-1" onkeypress={(e) => e.key === 'Escape' && (popShows = {})}>
        <div class="p-5 flex flex-col overflow-scroll scroll-auto h-full w-full z-50 shadow-2xl bg-neutral-200 dark:bg-neutral-900" transition:scale={{ duration: 200, easing: quartInOut }} onkeypress={(e) => e.key === 'Escape' && (popShows = {})} onclick={(e) => e.stopPropagation()} role="dialog" tabindex="-1">
            {@html currentFlow.value?.tasks[Object.entries(popShows).find(([k, v]) => v)[0]].output}
        </div>
    </div>
{/if}
