import { invoke } from '@tauri-apps/api/core';
import { auth } from '$lib/state/states.svelte';
import { emit } from '@tauri-apps/api/event';
import { type NHPromise } from '$lib/conn/Workflow.svelte';

type CommandResult = {
    stdout: string;
    stderr: string;
    exitCode: number;
};

// One-shot command. Use streamed command for best results.
export function Command(command: string): NHPromise<CommandResult> {
    try {
        let promise: NHPromise<CommandResult> = invoke('run_ssh_command', {
            command,
        });
        promise.cancel = () => {
            emit('abort_ssh_command').then(() => null);
        };
        return promise;
    } catch (error) {
        throw error;
    }
}
