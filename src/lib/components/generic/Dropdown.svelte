<script lang="ts">
    import { slide } from 'svelte/transition';
    import Button from '$lib/components/generic/Button.svelte';
    import threedots from "$lib/assets/threedots.svg"
    import type { SvelteComponent } from 'svelte';

    let { items, onselect: onSelect, class: buttonClass, direction = 'left', children = null } = $props();
    let open = $state(false);

    const toggle = () => (open = !open);
    const close = () => (open = false);
    const handleSelect = (item) => {
        onSelect(item);
        close();
    };
    const getDirection = () => {
        if (direction === 'right') return 'right-0';
        if (direction === 'top') return `bottom-[40px] -left-1.75`;
        return 'bottom-0';
    };
</script>

<div class="relative inline-block">
    {#if open}
        <div
        class="absolute {getDirection()} z-50 py-1 mx-2 mt-1 min-w-max overflow-auto max-h-96 bg-slate-300 dark:bg-slate-700 shadow-md" transition:slide={{ duration: 150 }}>
            {#each items as item, ind}
                <button class="block w-full text-left px-4 py-2 text-sm hover:bg-neutral-500/40 active:bg-neutral-400/40 transition-colors" onclick={() => handleSelect(ind)}>
                    {item}
                </button>
            {/each}
        </div>
    {/if}
    <Button transparent class="{!children && '[&]:px-0 [&]:py-3'} grid place-items-center transition-colors {open && '!bg-neutral-400/50'} {buttonClass}" onclick={toggle}>
        {#if children}
            {@render children?.()}
        {:else}
            <img src={threedots} alt="menu">
        {/if}
    </Button>
</div>
