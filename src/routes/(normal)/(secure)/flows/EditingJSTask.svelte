<script>
    import Editor from '$lib/components/Editor.svelte';
    import Button from '$lib/components/generic/Button.svelte';
    import { AsyncFunction } from '$lib/state/states.svelte.ts';
    import { alert } from '$lib/components/generic/Dialog.svelte';
    import { fade } from 'svelte/transition';
    import { createWindow } from '$lib/helpers/WindowCreator.ts';

    let { editingFlow = $bindable(), editingCurrentTask: i = $bindable() } = $props();

    const save = async () => {
        try {
            editingFlow.tasks[i].promise = new AsyncFunction('output', 'log', 'logError', 'fetchNoCors', editingFlow.tasks[i].promiseCode);
            i = -1;
        } catch (e) {
            await alert('Failed to save code', 'There was an error in your code: ' + e.message);
        }
    };
</script>

<div class="fixed top-0 left-0 w-screen h-screen bg-black/50 backdrop-blur-sm flex items-center justify-center z-50" transition:fade={{ duration: 100 }}>
    <div class="bg-neutral-800 p-5 rounded-2xl w-3/4 h-3/4 flex flex-col">
        <div class="flex flex-row justify-between items-center">
            <div class="text-2xl font-bold">{editingFlow.tasks[i].task}</div>
            <div class="flex flex-row gap-2">
                <Button transparent onclick={() => createWindow('/flows/jshelp')}>Ref.</Button>
                <Button onclick={() => save()}>Done</Button>
            </div>
        </div>
        <div class="m-2 p-2 bg-[rgb(30,30,30)] grow rounded-2xl">
            <Editor language="javascript" bind:value={editingFlow.tasks[i].promiseCode} class="grow p-2" />
        </div>
    </div>
</div>
