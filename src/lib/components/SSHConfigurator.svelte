<script lang="ts">
    import { onMount, tick } from "svelte";
    import Input from "./generic/Input.svelte";
    import { save, sshSettings } from "$lib/state/states.svelte";

    let external_identity = $state(false);
    let jump_server_selected = $state(false);

    let keyFile = $state('')
    let jumpServer = $state('')
    let jumpPassword = $state('')
    let switches = $state('')
    let keyPassphrase = $state('')

    $effect(() => {
        try {
            if (keyFile === '') sshSettings.value.keyFile = null
            else sshSettings.value.keyFile = keyFile
        } catch {}

        try {
            if (jumpServer === '') sshSettings.value.jumpServer = null
            else sshSettings.value.jumpServer = jumpServer
        } catch {}

        try {
            if (jumpPassword === '') sshSettings.value.jumpPassword = null
            else sshSettings.value.jumpPassword = jumpPassword
        } catch {}

        try {
            if (switches === '') sshSettings.value.switches = null
            else sshSettings.value.switches = switches
        } catch {}

        try {
            if (keyPassphrase === '') sshSettings.value.keyPassphrase = null
            else sshSettings.value.keyPassphrase = keyPassphrase
        } catch {}

        tick().then(async () => await save(sshSettings))
    })

    onMount(() => {
        if (sshSettings.value === null) sshSettings.value = {
            jumpServer: null,
            jumpPassword: null,
            keyFile: null,
            switches: null,
            keyPassphrase: null
        }
        else {
            keyFile = sshSettings.value.keyFile;
            jumpServer = sshSettings.value.jumpServer;
            jumpPassword = sshSettings.value.jumpPassword;
            switches = sshSettings.value.switches;
            keyPassphrase = sshSettings.value.keyPassphrase;
        }
    })
</script>

<div class="flex flex-col gap-2 p-2 bg-gray-900 w-full">
    <div class="text-sm px-2 font-bold">General Switches</div>
    <Input
        class="transition-all"
        type="password"
        name="SSH Key Passphrase"
        bind:value={keyPassphrase}
    />
    <div class="flex flex-row justify-center items-center gap-2 w-full">
        <Input
            name=""
            type="checkbox"
            class="w-min pl-2"
            bind:value={external_identity}
        />
        <div
            class="w-max shrink-0 font-bold transition-all {!external_identity &&
                'opacity-50 pointer-events-none'}"
        >
            -i
        </div>
        <Input
            bind:value={keyFile}
            class="transition-all {!external_identity &&
                'opacity-50 pointer-events-none'}"
            type="text"
            name="Absolute path to SSH key file"
        />
    </div>
    <div class="flex flex-row justify-center items-center gap-2 w-full">
        <Input
            name=""
            type="checkbox"
            class="w-min pl-2"
            bind:value={jump_server_selected}
        />
        <div
            class="w-max shrink-0 font-bold transition-all {!jump_server_selected &&
                'opacity-50 pointer-events-none'}"
        >
            -J
        </div>
        <Input
            class="transition-all {!jump_server_selected &&
                'opacity-50 pointer-events-none'}"
            type="text"
            name="Jump Server Name (username@server)"
            bind:value={jumpServer}
        />
        <Input
            class="transition-all {!jump_server_selected &&
                'opacity-50 pointer-events-none'}"
            type="password"
            name="Jump Server password"
            bind:value={jumpPassword}
        />
    </div>
    <Input
        class="transition-all"
        type="text"
        name="Other SSH switches"
        bind:value={switches}
    />
</div>
