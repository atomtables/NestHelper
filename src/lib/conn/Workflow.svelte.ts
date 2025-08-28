import {Channel, invoke} from "@tauri-apps/api/core";
import {auth} from "$lib/state/states.svelte.js";
import {emit} from "@tauri-apps/api/event";
import {LocalFetch} from "$lib/helpers/LocalFetch.js";

export type ProcessEvent = | {
    event: "started",
    data: {
        command: string,
        process_id: number
    }
} | {
    event: "output",
    data: {
        file: string, // as in stdout vs stderr
        output: string
    }
} | {
    event: "nextStage",
    data: {
        stage: number,
        file: string,
        output: string
    }
} | {
    event: "error",
    data: {
        stage: number,
        returnCode: number
    }
} | {
    event: "finished",
    data: {
        complete_output: string,
        returnCode: number
    }
}

export type Task = {
    task: string,
    description?: string,

    command: string,
    cwd?: string,
    frontend?: boolean,
    promise?: (
        outputs: {stdout: string, stderr: string}[],
        log: (str: string) => void,
        logError: (str: string) => void,
        fetchNoCors: typeof LocalFetch,
    ) => Promise<void>,
    promiseCode?: string,
    delay?: string, // seconds to wait after this task is done before moving on

    state?: "inactive" | "ongoing" | "done" | "failed",
    output?: string,
    stdout?: string,
    stderr?: string,
}

export interface NHPromise<T> extends Promise<T> {
    cancel?: () => void
}

export default class Workflow {
    public tasks: Task[] = $state<Task[]>([]);
    public complete: boolean = $state<boolean>(false);
    public failed: boolean = $state<boolean>(false);
    public started: boolean = $state<boolean>(false);

    public createdAt: Date = $state();
    public startedAt: Date = $state();
    public finishedAt: Date = $state();
    public name: string = $state()

    private promise: NHPromise<void>;
    private currentTask: number = $state(0);
    public task: Task = $derived(this.tasks?.[this.currentTask] || null)

    constructor(tasks: Task[], name: string) {
        if (!(this instanceof Workflow)) {
            throw new Error("Workflow must be instantiated with 'new'");
        }

        this.setTasks(tasks);

        this.complete = false;
        this.failed = false;
        this.promise = null;
        this.createdAt = new Date();
        this.name = name;

        this.currentTask = 0;
    }
    setTasks(tasks: Task[]) {
        this.tasks = tasks.map((task): Task => ({
            ...task,
            state: "inactive",
            output: `<i>${task.command || ''}</i>`,
            stdout: ``,
            stderr: ``
        }));
        this.tasks.unshift({ command: null, task: "Connecting via SSH", state: "inactive", output: "", stdout: ``, stderr: `` });
    }

    start() {
        globalThis.flow = this;
        this.started = true;
        this.currentTask = 0;
        this.complete = false;
        this.failed = false;
        this.startedAt = new Date();
        this.finishedAt = null;

        let onEvent = new Channel<ProcessEvent>();
        onEvent.onmessage = async (message) => {
            if (message.event === 'started') {
                this.tasks[this.currentTask].state = "ongoing";
                this.tasks[this.currentTask].output = `<i>${this.tasks[this.currentTask].command || ''}</i>`;
            } else if (message.event === 'output') {
                if (message.data.file === 'stderr') {
                    this.tasks[this.currentTask].output += `<div class="text-red-500">${message.data.output}</div>`;
                    this.tasks[this.currentTask].stderr += message.data.output + "\n";
                } else {
                    this.tasks[this.currentTask].output += `<div>${message.data.output}</div>`;
                    this.tasks[this.currentTask].stdout += message.data.output + "\n";
                }
            } else if (message.event === 'nextStage') {
                this.tasks[this.currentTask].state = "done";
                this.currentTask++;
                if (this.tasks[this.currentTask]) {
                    this.tasks[this.currentTask].state = "ongoing"

                    if (this.tasks[this.currentTask].frontend) {
                        this.tasks[this.currentTask].output = `<i>This task is handled by NestHelper and has no significant output.</i>`
                        await Promise.resolve(
                            this.tasks[this.currentTask].promise(
                                this.tasks.map(t => ({
                                    stdout: t.stdout,
                                    stderr: t.stderr
                                })),
                                str => this.tasks[this.currentTask].output += `<div>${str}</div>`,
                                str => this.tasks[this.currentTask].output += `<div class="text-red-500">${str}</div>`,
                                LocalFetch
                            )
                        )
                            .then(() => {
                                emit("ready_to_move_on")
                            })
                            .catch((err) => {
                                this.tasks[this.currentTask].output += `<div class="text-red-500">Error in frontend task: ${err.message}</div>`;
                                emit("error_on_the_frontend")
                            })
                    }
                }
            } else if (message.event === 'error') {
                this.tasks[this.currentTask].state = "failed";
                this.failed = true;
                this.started = true;
                this.finishedAt = new Date();
            } else if (message.event === 'finished') {
                this.complete = true;
                setTimeout(() => this.started = false, 3000)
                if (this.tasks[this.currentTask])
                    this.tasks[this.currentTask].state = "done";
                this.finishedAt = new Date();
            } else {
                console.warn("Unknown event:", message);
            }
        };

        this.promise = invoke("run_ssh_flow", {
            username: auth.value.username,
            commands: this.tasks.map(t => ({
                command: t.command,
                frontend: t.frontend || false,
                cwd: t.cwd,
                delay: t.delay,
            })).filter(c => c),
            onEvent
        })

        this.promise.cancel = () => {
            emit("abort_ssh_flow").catch(err => {
                console.error("Failed to cancel SSH flow:", err);
            });
            this.started = false;
        }
    }
}