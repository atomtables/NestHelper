import {type NHPromise, type ProcessEvent} from "$lib/conn/Workflow.svelte";
import {Channel, invoke} from "@tauri-apps/api/core";
import {auth} from "$lib/state/states.svelte";
import {emit} from "@tauri-apps/api/event";

export class StreamedCommandSvelte {
    public id: string = crypto.randomUUID();
    public command: string;
    public state: "inactive" | "ongoing" | "done" | "failed" = $state("inactive");
    // Output as HTML string.
    public htmlOutput: string = $state("");
    public output: { file: "stdout" | "stderr", output: string }[] = $state([]);
    public stdout: string = $state("");
    public stderr: string = $state("");
    public promise: NHPromise<void>;
    public returnCode: number = $state(null);

    private channel: Channel<ProcessEvent>

    constructor(command: string) {
        if (!(this instanceof StreamedCommandSvelte)) {
            throw new Error("StreamedCommandSvelte must be instantiated with 'new'");
        }

        this.command = command;
        this.state = "inactive";
        this.htmlOutput = "";
        this.stdout = ``;
        this.stderr = ``;
    }

    reset() {
        this.state = "inactive";
        this.htmlOutput = "";
        this.stdout = ``;
        this.stderr = ``;
    }

    start() {
        this.channel = new Channel<ProcessEvent>();
        this.channel.onmessage = message => {
            if (message.event === "started") {
                this.state = "ongoing";
            } else if (message.event === "output") {
                if (message.data.file === "stdout") {
                    this.stdout += message.data.output;
                    this.htmlOutput += `<div class="">${message.data.output}</div>`;
                    this.output.push({file: "stdout", output: message.data.output});
                } else if (message.data.file === "stderr") {
                    this.stderr += message.data.output;
                    this.htmlOutput += `<div class="text-red-500">${message.data.output}</div>`;
                    this.output.push({file: "stderr", output: message.data.output});
                }
            } else if (message.event === "error") {
                this.returnCode = message.data.returnCode;
                this.state = "failed";
            } else if (message.event === "finished") {
                this.returnCode = message.data.returnCode;
                this.state = "done";
            }
        }
        this.promise = invoke("run_ssh_command_with_stream", {
            command: this.command,
            onEvent: this.channel
        })
        this.promise.cancel = () => emit("abort_ssh_command");
    }
}