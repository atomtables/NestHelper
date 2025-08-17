<script>
    import Spinner from "$lib/components/Spinner.svelte";

    let {children, onclick, class: className, type, disabled, transparent, disableLoading, ...etc} = $props();

    let resolving = $state(false);

    const handleClick = async () => {
        if (disabled || resolving) return;

        resolving = true;
        try {
            await Promise.resolve(onclick?.(new CustomEvent(`onClickButton-${name}`)));
        } finally {
            resolving = false;
        }
    };
</script>

<button {type} {...etc}
        class="rounded-full grid place-items-center px-5 py-2 {disabled ? `cursor-not-allowed ${!transparent && 'dark:bg-gray-700 bg-purple-200'} text-gray-300` : `cursor-pointer ${!transparent ? 'dark:bg-purple-700 dark:hover:bg-purple-600 dark:active:bg-purple-500 bg-purple-300 hover:bg-purple-400 active:bg-purple-500' : 'hover:bg-neutral-400/25 active:bg-neutral-400/50'}`} {resolving && !disableLoading && 'cursor-progress'} font-bold transition-all flex flex-row {className}"
        onclick={handleClick}>
    <span class="flex flex-row">
        {#if resolving && !disableLoading}
            <Spinner size="24" class="mr-2" type={!transparent ? 'secondary' : 'primary'}/>
        {/if}
        {@render children?.()}
    </span>
</button>