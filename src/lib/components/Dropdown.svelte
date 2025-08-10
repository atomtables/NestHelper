<script>
    import { slide } from 'svelte/transition';
    import Button from "$lib/components/Button.svelte";

    let { items, onselect: onSelect, class: buttonClass, buttonText, direction } = $props();
    let open = $state(false);

    const toggle = () => (open = !open);
    const close = () => (open = false);
    const handleSelect = (item) => {
        onSelect(item);
        close();
    };
    const getDirection = () => {
        if (direction === "right") return 'right-0';
        return ''
    }
</script>

<div class="relative inline-block">
    <Button
            transparent
            class="{!buttonText && '[&]:px-0 [&]:py-3'} grid place-items-center transition-colors {open && '!bg-neutral-400/50'} {buttonClass}"
            onclick={toggle}
    >
        {#if buttonText}
            {@html buttonText}
        {:else}
            <span class="material-symbols-outlined icons-fill text-xl">more_vert</span>
        {/if}
    </Button>

    {#if open}
        <div
                class="absolute {getDirection()} z-50 py-1 min-w-max overflow-hidden bg-slate-300 dark:bg-slate-700 shadow-md"
                transition:slide={{ duration: 150 }}
        >
            {#each items as item, ind}
                <button
                        class="block w-full text-left px-4 py-2 text-sm hover:bg-neutral-500/40 active:bg-neutral-400/40 transition-colors"
                        onclick={() => handleSelect(ind)}
                >
                    {item}
                </button>
            {/each}
        </div>
    {/if}
</div>