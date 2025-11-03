<script lang="ts">
    import Button from '$lib/components/generic/Button.svelte';
    import { onMount } from 'svelte';
    import { fade, scale, slide } from 'svelte/transition';

    let { file, mouseX, mouseY, onoptionclicked, options } = $props();

    let hide = $state(true);

    const handle = (v) => {
        hide = true;
        setTimeout(() => onoptionclicked?.(v), 200)
    };
    onMount(() => {
        hide = false
    })
</script>

<svelte:window onmousedown={() => handle(null)} onscrollcapture={() => handle(null)} />

{#if !hide}
    <div class="fixed flex-col flex p-1 rounded-tl-md rounded-2xl bg-slate-900" style="top: {mouseY}px; left: {mouseX}px;" transition:slide={{ duration: 200 }}>
        {#each options as option, i}
            <div onmousedown={(e) => { e.stopPropagation(); }} role="none">
                <Button transparent class="text-sm !py-1 !px-3" onclick={(e) => { e.stopPropagation(); handle(i); }}>
                    {option}
                </Button>
            </div>
        {/each}
    </div>
{/if}
