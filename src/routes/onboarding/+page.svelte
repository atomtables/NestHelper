<script>
    import {quintOut} from "svelte/easing";
    import {onMount, tick} from "svelte";
    import {scale, slide} from "svelte/transition";
    import image from "$lib/assets/favicon.png"
    import Button from "$lib/components/Button.svelte";
    import Input from "$lib/components/Input.svelte";
    import {auth, save} from "$lib/state/states.svelte.js";
    import Flow from "$lib/components/Flow.svelte";
    import {Channel, invoke} from "@tauri-apps/api/core";
    import {goto} from "$app/navigation";

    let showChildren = $state(false);
    let screen = $state(0);

    let username = $state("");
    let disclaimer = $state(false);

    function scaleCircle(node, { delay = 0, duration = 700, easing = quintOut }) {
        // noinspection JSUnusedGlobalSymbols
        return {
            delay,
            duration,
            easing,
            css: t => {
                return `
                    transform: scale(${t * 1.5});
                    border-radius: 1000px;
                `;
            }
        }
    }
    onMount(() => {
        setTimeout(() => showChildren = true, 700)
    })

    let result = $state();
    let tasks = $state([
        { command: null, task: "Connecting via SSH" },
        { command: "nest caddy list", task: "Getting connected domains" },
        { command: "cat Caddyfile", task: "Getting Caddyfile" },
        { command: "nest resources", task: "Getting nest resources" },
        { command: null, frontend: true, task: "Finalising setup" }
    ])
    let currentTask = $state(0);

    let handler = async () => {
        if (screen === 2) {
            await tick();
            if (!username || !disclaimer) {
                return;
            }
            auth.value.username = username;
            await save(auth);
            await tick();

            const onEvent = new Channel();
            onEvent.onmessage = (message) => {
                if (message.event === 'started') {
                    tasks[currentTask].state = "ongoing";
                    tasks[currentTask].output = ""
                } else if (message.event === 'output') {
                    if (message.data.file === 'stderr') {
                        tasks[currentTask].output += `<div class="text-red-500">${message.data.output}</div>`;
                    } else {
                        tasks[currentTask].output += `<div>${message.data.output}</div>`;
                    }
                } else if (message.event === 'nextStage') {
                    tasks[currentTask].state = "done";
                    currentTask++;
                    if (tasks[currentTask]) {
                        tasks[currentTask].state = "ongoing"
                        tasks[currentTask].output = "";
                    }
                } else if (message.event === 'error') {
                    tasks[currentTask].state = "failed";
                } else if (message.event === 'finished') {
                    tasks[currentTask].state = "done";
                } else {
                    console.warn("Unknown event:", message);
                }
                console.log(message);
            };
            invoke("run_ssh_flow", {
                username: auth.value.username,
                commands: tasks.map(t => t.command).filter(c => c),
                onEvent
            }).then(r => result = r);
            onEvent.send();
        } else if (screen === 3) {
            showChildren = false;
            setTimeout(() => goto("/main"), 1000);
        }
    }
</script>

<div class="h-full w-full bg-purple-200 dark:bg-purple-900 overflow-y-scroll" in:scaleCircle out:scaleCircle>
    {#if showChildren}
        <div class="flex flex-col max-w-4xl mx-auto h-full items-start justify-center p-5 gap-4 overflow-y-scroll" transition:scale>
            <div class="flex flex-row items-center gap-4 justify-center backdrop-blur-md">
                <img src={image} class="h-16 w-16" alt="app icon">
                <div class="text-4xl font-sans">NestHelper</div>
            </div>
            {#if screen === 0}
                <div class="flex flex-col gap-4" transition:slide>
                    <div>
                        NestHelper is a program designed to help you manage your Nest account more easily.
                        <br><br>
                        It often takes a while to do things on Nest, with all the lag and such, so setting up a server to
                        run might start to get pretty annoying.
                        <br><br>
                        NestHelper solves this by allowing you to make all your changes before committing them to the server,
                        allowing you to go get a cup of coffee while your latest commit gets pulled and set up on the server.
                    </div>
                </div>
            {:else if screen === 1}
                <div class="flex flex-col items-start gap-2" transition:slide>
                    <div>
                        Warnings:
                        <ul class="list-disc pl-5 pt-2 text-sm">
                            <li><b>NestHelper is by no means a sanctioned tool</b>, meaning that if a glitch occurs or
                                someone gets mad (or you do something bad), your use of NestHelper may result in
                                consequences such as a ban. You have been warned.</li>
                            <li>It is recommended that you access Nest via SSH at least once before using
                                NestHelper to ensure there are no authentication errors.</li>
                            <li>Make sure that the you enter your current username and your SSH key is stored
                                in your local SSH folder. Too many failed login attempts
                                may get you temporarily IP banned from Nest and Nest services.</li>
                        </ul>
                    </div>
                    <div class="flex flex-row justify-center items-center gap-2">
                        <Input bind:value={username} name="Username" id="username" type="text"
                               className="w-full max-w-md" />
                        <div>@hackclub.app</div>
                    </div>
                    <div class="flex flex-row justify-center items-center gap-2">
                        <Input type="checkbox" class="w-min" bind:value={disclaimer} />
                        <div class="text-xs {!disclaimer && 'animate-pulse'}">
                            I agree to all the warnings as set above and fully maintain that any consequences from the
                            use of this tool are my own responsibility. I have been warned against using this tool to
                            break the Nest terms of service and I will not use it to do so. I will be responsible and
                            ensure that I do not abuse this tool or distribute a version that will result in abuse.
                        </div>
                    </div>
                </div>
            {:else if screen === 2}
                <div transition:slide class="w-full">
                    <Flow bind:tasks />
                </div>
            {/if}
            <div class="flex flex-row justify-between w-full items-center">
                <Button
                    onclick={() => screen--}
                    disabled={screen === 0}
                >Back</Button>
                <Button
                    onclick={async () => {await tick(); screen++; await handler(); await tick();}}
                    disabled={
                        screen >= 5 ||
                        (screen === 1 && (!username || !disclaimer))
                    }
                >Continue</Button>
            </div>
        </div>
    {/if}
</div>