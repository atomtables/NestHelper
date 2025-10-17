<script>
    import { quartInOut, quintIn } from 'svelte/easing';
    import Spinner from '$lib/components/generic/Spinner.svelte';
    import Button from '$lib/components/generic/Button.svelte';
    import { slide } from 'svelte/transition';
    import Input from '$lib/components/generic/Input.svelte';
    import trash from '$lib/assets/trash.png';
    import { app, auth, save } from '$lib/state/states.svelte.ts';
    import { onMount, onDestroy, tick } from 'svelte';
    import command from '$lib/assets/command.png';
    import frontend from '$lib/assets/frontend.png';
    import { flip } from 'svelte/animate';
    import { confirm, alert } from '$lib/components/generic/Dialog.svelte';
    import { beforeNavigate, goto } from '$app/navigation';
    import { userflows, load } from '$lib/state/states.svelte.ts';
    import Editor from '$lib/components/Editor.svelte';
    import EditingJSTask from './EditingJSTask.svelte';

    let { editingFlow = $bindable() } = $props();

    let ignore = false;
    let saved = $state(true);
    $effect(() => {
        app.value.status = saved ? 'All changes saved' : 'Unsaved changes';
    });
    onMount(() => {
        app.value.status = 'Editing flow';
        app.value.pageActions.push(['back', leave]);

        beforeNavigate(async (e) => {
            if (!saved) {
                if (!ignore) e.cancel();
                let result;
                if (!ignore) result = (await confirm('Are you sure you want to leave the page?', 'You are currently editing a flow that has not been saved. Your changes will be lost.'))[0];
                if (result) {
                    ignore = true;
                    await goto(e.to.url.href);
                    await load(userflows);
                }
            }
        });
    });
    const leave = async () => {
        if (editingFlow.draft) {
            let [result] = await confirm('Are you sure you want to leave?', 'This flow has not been saved and will be lost.');
            if (result) {
                userflows.value.flows = userflows.value.flows.filter((v) => JSON.stringify(v) !== JSON.stringify(editingFlow));
                editingFlow = null;
            }
        } else if (!saved) {
            let [result] = await confirm('Are you sure you want to leave?', 'Your changes have not been saved and will be lost.');
            if (result) {
                editingFlow = null;
                await load(userflows);
            }
        } else {
            editingFlow = null;
        }
    };
    const verifyIntegrity = () => {
        if (!editingFlow.name || editingFlow.name.trim() === '') return false;
        for (let task of editingFlow.tasks) {
            if (!task.task || task.task.trim() === '') return false;
            if (!task.frontend) {
                if (!task.command || task.command.trim() === '') return false;
                if (!task.cwd || task.cwd.trim() === '') return false;
            }
        }
        return true;
    };
    onDestroy(() => {
        app.value.pageActions = [];
        app.value.status = '';
    });
    let scrolling;
    let editingCurrentTask = $state(-1);
    const editJS = async (index) => {
        let [result] = await confirm(
            'WARNING: DISCLAIMER',
            'Running incorrectly formed arbitrary JavaScript code on the frontend can severely damage the functionality of the app, resulting ' +
                'in code that may cause devastating bugs on your Nest account. Moreover, you should NEVER run code that anyone ' +
                'has sent you without verifying it 3 times. Make sure your code does what you want it to, NestHelper carries no ' +
                'responsibility for verifying the quality or security of your JavaScript.'
        );
        if (!result) return;
        editingCurrentTask = index;
    };
</script>

<div class="w-full h-full flex flex-row flex-nowrap" transition:slide>
    <div class="grow overflow-y-auto h-full bg-neutral-800 p-10" bind:this={scrolling}>
        {#if editingFlow.tasks.length > 0}
            <div class="flex flex-col gap-2" transition:slide>
                {#each editingFlow.tasks as task, i (i)}
                    <div class="relative flex flex-col items-start gap-2 w-full dark:bg-purple-900 bg-purple-300 p-2 rounded-2xl" transition:slide animate:flip>
                        <div class="flex flex-col items-start gap-4 p-5 w-full">
                            <div class="-my-4 w-full">
                                <Input action={() => (saved = false)} name="Task name" bind:value={task.task} class="font-bold text-xl w-full" />
                            </div>
                            <div class="-my-3 w-full">
                                <Input action={() => (saved = false)} name="Task description" bind:value={task.description} class="font-bold text-sm w-full" />
                            </div>
                            {#if task.frontend}
                                <div>This task is performed as a Javascript function on the frontend.</div>
                                <div class="flex flex-row justify-end items-center w-full gap-2">
                                    <Button onclick={() => editJS(i)}>Edit Code</Button>
                                    <Button destructive onclick={() => ((saved = false), editingFlow.tasks.splice(i, 1))}>
                                        <img src={trash} alt="delete" class="h-6" />
                                    </Button>
                                </div>
                            {:else}
                                <div class="-mb-3">This task runs using SSH on your Nest account.</div>
                                <div class="flex flex-row justify-between items-center w-full gap-2">
                                    <Input action={() => (saved = false)} autocorrect="off" autocapitalize="off" spellcheck="false" name="Command" bind:value={task.command} />
                                </div>
                                <div class="flex flex-row items-center justify-end w-full gap-5 -mt-5">
                                    <Input action={() => (saved = false)} name="Working directory" bind:value={task.cwd} class="text-xs" />
                                    <Input action={() => (saved = false)} name="Postdelay (s)" bind:value={task.delay} class="text-xs w-30!" />
                                    <Button destructive onclick={() => ((saved = false), editingFlow.tasks.splice(i, 1))}>
                                        <img src={trash} alt="delete" class="h-6" />
                                    </Button>
                                </div>
                            {/if}
                        </div>
                    </div>
                {/each}
            </div>
        {:else}
            <div class="w-full h-full flex flex-col items-center justify-center gap-5">
                <div class="text-2xl font-bold">This flow has no tasks.</div>
                <div class="text-neutral-400">Use the sidebar to add more tasks.</div>
            </div>
        {/if}
    </div>
    <div class="shrink-0 w-96 h-full bg-purple-900 p-2 flex flex-col justify-between items-end">
        <div class="flex flex-col w-full gap-2">
            <Input action={() => (saved = false)} name="Flow name" bind:value={editingFlow.name} class="font-bold text-2xl! w-full" />
            <button
                onclick={() => {
                    saved = false;
                    editingFlow.tasks.push({
                        description: '',
                        task: 'New Task',
                        frontend: false,
                        command: '',
                        cwd: `/home/${auth.value.username}`,
                    });
                    setTimeout(() => {
                        // scroll smoothly
                        tick().then(() => scrolling.scrollTo({ top: scrolling.scrollHeight, behavior: 'smooth' }));
                    }, 300);
                }}
                class="w-full bg-purple-800 hover:bg-purple-700 active:bg-purple-600 cursor-pointer transition-all rounded-full flex flex-row items-center justify-start p-2 gap-2"
            >
                <!-- plus svg -->
                <span class="h-8 w-8 bg-purple-500 shadow-2xl shadow-black rounded-full flex items-center justify-center">
                    <img src={command} alt="SSH command" class="w-5" />
                </span>
                <span> New SSH Command Task </span>
            </button>
            <button
                onclick={() => {
                    saved = false;
                    editingFlow.tasks.push({
                        description: '',
                        task: 'New Task',
                        frontend: true,
                        command: null,
                    });
                    setTimeout(() => {
                        // scroll smoothly
                        tick().then(() => scrolling.scrollTo({ top: scrolling.scrollHeight, behavior: 'smooth' }));
                    }, 300);
                }}
                class="w-full bg-purple-800 hover:bg-purple-700 active:bg-purple-600 cursor-pointer transition-all rounded-full flex flex-row items-center justify-start p-2 gap-2"
            >
                <!-- plus svg -->
                <span class="h-8 w-8 bg-purple-500 shadow-2xl shadow-black rounded-full flex items-center justify-center">
                    <img src={frontend} alt="SSH command" class="w-5" />
                </span>
                <span> New Frontend JS Task </span>
            </button>
        </div>
        <div class="flex flex-row gap-2">
            <Button
                destructive
                onclick={async () => {
                    let [result] = await confirm('Delete Flow', 'Are you sure you would like to delete this flow? This action cannot be undone.');
                    if (result) {
                        userflows.value.flows = userflows.value.flows.filter((v) => JSON.stringify(v) !== JSON.stringify(editingFlow));
                        userflows.set = true;
                        await save(userflows);
                    }
                    editingFlow = null;
                }}
            >
                <img src={trash} alt="delete" class="h-6" />
            </Button>
            <Button
                onclick={async () => {
                    if (!verifyIntegrity()) {
                        await alert('Error while saving', 'Fill out the name of your task, commands, and working directories before saving.');
                        return;
                    }
                    if (editingFlow.draft) editingFlow.draft = false;
                    await save(userflows);
                    saved = true;
                }}>Save</Button
            >
            <Button
                onclick={async () => {
                    if (!verifyIntegrity()) {
                        await alert('Error while saving', 'Fill out the name of your task, commands, and working directories before saving.');
                        return;
                    }
                    if (editingFlow.draft) editingFlow.draft = false;
                    editingFlow.lastUpdated = new Date();
                    await save(userflows);
                    saved = true;
                    editingFlow = null;
                }}>Save and Exit</Button
            >
        </div>
    </div>
</div>

{#if editingCurrentTask > -1}
    <EditingJSTask bind:editingFlow bind:editingCurrentTask />
{/if}
