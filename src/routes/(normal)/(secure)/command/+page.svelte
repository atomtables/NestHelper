<script>
    import {flip} from "svelte/animate";
    import Input from "$lib/components/Input.svelte";
    import Button from "$lib/components/Button.svelte";
    import {StreamedCommandSvelte} from "$lib/conn/StreamedCommand.svelte.ts";
    import Spinner from "$lib/components/Spinner.svelte";
    import {commandHistory, filesystem} from "$lib/state/states.svelte.ts";
    import {onMount} from "svelte";
    import stop from "$lib/assets/stop.png";
    import save from "$lib/assets/save.png";
    import trash from "$lib/assets/trash.png";
    import {save as saveDialog} from '@tauri-apps/plugin-dialog';
    import {save as saveState} from "$lib/state/states.svelte.ts"
    import CommandOutput from "./CommandOutput.svelte";
    import { slide } from "svelte/transition"
    import {create} from "@tauri-apps/plugin-fs";

    let value = $state('')
    let currentCommand = $state();

    onMount(() => {
        // commandHistory.value =  []
    })
    const submit = () => {
        currentCommand = new StreamedCommandSvelte(value)
        currentCommand.start();
        commandHistory.value.history = (commandHistory.value.history instanceof Array && commandHistory.value.history) || []
        let ind = commandHistory.value.history.push({
            id: currentCommand.id,
            command: value,
            timestamp: new Date()
        }) - 1
        let history = commandHistory.value.history
        currentCommand.promise.then(() => {
            history[ind].returnCode = currentCommand.returnCode
            history[ind].stdout = currentCommand.stdout
            history[ind].stderr = currentCommand.stderr
            history[ind].output = currentCommand.htmlOutput
            history[ind].outputArr = currentCommand.output
        }).catch(() => {
            history[ind].returnCode = currentCommand.returnCode
            history[ind].stdout = currentCommand.stdout
            history[ind].stderr = currentCommand.stderr
            history[ind].outputArr = currentCommand.output
            history[ind].output = currentCommand.htmlOutput
        }).finally(() => {
            saveState(commandHistory)
        })
    }
    const reversed = arr => {
        let newArr = []
        for (let i = arr.length - 1; i >= 0; i--) {
            newArr.push(arr[i])
        }
        return newArr
    }
    const dateIsSameDay = (d1, d2) => {
        return d1.getFullYear() === d2.getFullYear() &&
            d1.getMonth() === d2.getMonth() &&
            d1.getDate() === d2.getDate();
    }

    let scrollContainer;
    $effect(() => {
        if (currentCommand?.output.length && scrollContainer) {
            scrollContainer.scrollTop = scrollContainer.scrollHeight;
        }
    })

    const writeOutputToFile = async (command) => {
        const path = await saveDialog({
            defaultPath: `output-${command.command.replace(' ', '_')}.txt`,
            canCreateDirectories: true
        });

        try {
            const file = await create(path);
            let arr = new Uint8Array();
            command.output.forEach(({file, output}) => {
                let encoder = new TextEncoder();
                let encoded = encoder.encode(output + '\n');
                let newArr = new Uint8Array(arr.length + encoded.length);
                newArr.set(arr);
                newArr.set(encoded, arr.length);
                arr = newArr;
            });
            await file.write(arr);
            await file.close();
        } catch (e) {
            console.error("Error saving file:", e);
            await alert("Error", `There was an error saving the file: ${e.message}.`);
        }
    }
</script>

<div class="w-full h-full">
    <div class="w-full h-full flex flex-row flex-nowrap">
        <div class="flex-1/4 flex flex-col items-start w-full gap-0 p-2 bg-purple-900 overflow-y-auto scroll-y-auto">
            <div class="p-2 font-bold">
                Previous Commands
            </div>
            {#if commandHistory.value.history && commandHistory.value.history instanceof Array}
                {@const history = commandHistory.value.history}
                {#each reversed(history) as cmd, i (history.length - i)}
                    <button     animate:flip={{duration: 300}}
                         class="pl-2 max-w-full w-full flex flex-col items-start
                         hover:bg-purple-800 active:bg-purple-700 transition-all
                         rounded-lg cursor-pointer p-1 text-left overflow-x-clip text-nowrap text-ellipsis"
                         onclick={() => {
                            if (cmd?.returnCode === undefined) return;
                            currentCommand = new StreamedCommandSvelte(cmd.command)
                            currentCommand.id = cmd.id
                            currentCommand.returnCode = cmd.returnCode
                            currentCommand.stdout = cmd.stdout
                            currentCommand.stderr = cmd.stderr
                            currentCommand.output = cmd.outputArr
                            value = cmd.command
                        }}
                    >
                    <span class="text-xs text-gray-300">
                        {dateIsSameDay(new Date(), new Date(cmd.timestamp)) ? new Date(cmd.timestamp).toLocaleTimeString() : new Date(cmd.timestamp).toLocaleString()}
                        • {@html cmd?.returnCode === 0 ? "<span class=\"text-green-400\">success</span>" : cmd?.returnCode !== undefined ? `<span class="text-red-400">exit code ${cmd.returnCode}</span>` : '<span class="text-yellow-400">running</span>'}
                    </span>
                        <span class="flex flex-row items-center gap-2">
                        {#if cmd?.returnCode === undefined}
                            <Spinner size="24px" />
                        {/if}
                            {cmd.command}
                    </span>
                    </button>
                {/each}
            {:else}
                no history
            {/if}
        </div>
        <div class="flex flex-col items-center w-full flex-4/5">
            <div class="flex flex-col w-full grow overflow-y-scroll scroll-auto" bind:this={scrollContainer}>
                {#if currentCommand}
                    <CommandOutput bind:command={currentCommand} />
                {:else}
                    <div class="w-full h-full flex flex-col items-center justify-center">
                        <div class="text-2xl">
                            Hey, forget that delay typing into SSH.
                        </div>
                        <div class="text-base text-gray-400">
                            Just run commands directly here.
                        </div>
                    </div>
                {/if}
            </div>
            <div class="w-full px-4 shrink-0">
                <div class="bg-neutral-900 w-full flex flex-col items-center justify-center rounded-t-xl px-2">
                    <div class="flex flex-row items-center justify-start gap-2 w-full pt-2 transition-all">
                        {#if currentCommand?.state === 'ongoing'}
                            <div transition:slide={{ duration: 150 }}>
                                <button onclick={() => currentCommand.promise.cancel()} class="group relative w-8 h-8 bg-purple-800 hover:bg-purple-700 active:bg-purple-600 cursor-pointer flex items-center justify-center rounded-full">
                                    <img src={stop} alt="stop current process" class="w-4" />
                                    <span class="z-50 text-nowrap absolute -top-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                                    Stop command
                                </span>
                                </button>
                            </div>
                        {/if}
                        {#if currentCommand?.returnCode !== undefined}
                            <div transition:slide={{ duration: 150 }} >
                                <button onclick={() => writeOutputToFile(currentCommand)} class="group relative w-8 h-8 bg-purple-800 hover:bg-purple-700 active:bg-purple-600 cursor-pointer flex items-center justify-center rounded-full">
                                    <img src={save} alt="stop current process" class="w-4" />
                                    <span class="z-50 text-nowrap absolute -top-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                                    Save output
                                </span>
                                </button>
                            </div>
                        {/if}
                        {#if currentCommand && currentCommand?.state !== "ongoing"}
                            <div transition:slide={{ duration: 150 }} >
                                <button onclick={() => {commandHistory.value.history = commandHistory.value.history.filter(v => v.id !== currentCommand.id); currentCommand = null}} class="group relative w-8 h-8 bg-purple-800 hover:bg-purple-700 active:bg-purple-600 cursor-pointer flex items-center justify-center rounded-full">
                                    <img src={trash} alt="stop current process" class="w-4" />
                                    <span class="z-50 text-nowrap absolute -top-10 py-1 px-2 left-1/2 -translate-x-1/2 bg-gray-700 opacity-0 invisible group-hover:visible group-hover:opacity-100 group-hover:delay-150 duration-150 shadow-2xl transition-all">
                                    Delete from history
                                </span>
                                </button>
                            </div>
                        {/if}
                    </div>
                    <div class="flex flex-row items-center justify-center gap-2 w-full">
                        <Input autocorrect="off" bind:value name="Command" />
                        <Button onclick={submit} class="" disabled={value.length < 1}>Submit</Button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>