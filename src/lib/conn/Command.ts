import {invoke} from "@tauri-apps/api/core";
import {auth} from "$lib/state/states.svelte";

type CommandResult = {
    stdout: string,
    stderr: string,
    exitCode: number
}

// One-shot command.
export async function Command(command: string): Promise<CommandResult> {
    try {
        return await invoke("run_ssh_command", {
            username: auth.value.username,
            command
        });
    } catch (error) {
        throw error;
    }
}