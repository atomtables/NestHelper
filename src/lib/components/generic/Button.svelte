<script>
    import Spinner from '$lib/components/generic/Spinner.svelte';

    let {
        children,
        onclick,
        class: className = '',
        type = 'button',
        disabled = $bindable(false),
        transparent = false,
        destructive = false,
        disableLoading = false,
        ...etc
    } = $props();

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

<button
    {type}
    {...etc}
    class="rounded-full px-5 py-2 {disabled
        ? `cursor-not-allowed ${!transparent && 'dark:bg-gray-700 bg-purple-200'} text-gray-300`
        : `cursor-pointer ${!transparent ? 'dark:bg-purple-700 dark:hover:bg-purple-600 dark:active:bg-purple-500 bg-purple-300 hover:bg-purple-400 active:bg-purple-500' : 'hover:bg-neutral-400/25 active:bg-neutral-400/50'}`} {destructive &&
        'dark:bg-red-700 dark:hover:bg-red-600 dark:active:bg-red-500 bg-red-300 hover:bg-red-400 active:bg-red-500'} {resolving &&
        !disableLoading &&
        'cursor-progress'} font-bold transition-all flex flex-row items-center justify-center {className}"
    onclick={handleClick}
>
    <span class="flex flex-row">
        {#if resolving && !disableLoading}
            <Spinner size="24" class="mr-2" type={!transparent ? 'secondary' : 'primary'} />
        {/if}
        {@render children?.()}
    </span>
</button>
