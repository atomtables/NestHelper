<script>
    let { command = $bindable() } = $props();
</script>

<div class="w-full flex justify-center p-5">
    <div class="bg-neutral-800 p-5 rounded-2xl w-full">
        <div class="font-mono whitespace-pre-wrap break-all overflow-x-hidden w-full">
            {#each command.output as { file, output }, i (i)}
                {@const isLast = i === command.output.length - 1 && command.state === 'ongoing'}
                <div class="flex">
                    <div class="text-gray-500 w-8 text-right pr-2 select-none">{i + 1}</div>
                    <div class="flex-1 {file !== 'stdout' ? 'text-red-500' : ''}">
                        {@html output}{#if isLast}<span class="animate-pulse">_</span>{/if}
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
