import { caddy as caddyStore, saveAll, server as serverStore, services as servicesStore, auth, userflows, commandHistory } from '$lib/state/states.svelte.js';
import { type Task } from '$lib/conn/Workflow.svelte.js';
import { attempt } from '$lib/helpers/attempt.js';
import { filesystem } from '$lib/state/states.svelte';
import { EditFile } from '$lib/conn/FIle';

const caddy: Task[] = [
    { command: 'nest caddy list', task: 'Getting connected domains' },
    { command: 'cat Caddyfile', task: 'Getting Caddyfile' },
    { command: 'caddy adapt', task: 'Getting Caddy rules in JSON' },
];

const server: Task[] = [
    { command: 'nest resources', task: 'Getting current resource usage' },
    {
        command: 'uptime && echo --separate-- && uptime -s',
        task: 'Getting server stats',
    },
];

const startup: () => Task[] = () => [
    ...caddy,
    ...server,
    {
        command: `for file in ~/.config/systemd/user/*; do if [ -f "$file" ]; then systemctl --user status "$\{file#/home/${auth.value.username}/.config/systemd/user/}" | grep -e Loaded -e Active -e ● -e × -e ○ -e "Main PID"; fi; done`,
        task: 'Getting systemctl services',
    },
    {
        command: null,
        frontend: true,
        promise: async (outpu, self) => {
            if (!filesystem.value || !filesystem.value?.files) {
                filesystem.value = {
                    files: {},
                    fileData: {},
                    lastUpdated: new Date(0),
                    filesWereModified: false,
                    currentFolder: [],
                };
            }
            if (!userflows.value || !userflows.value?.flows) {
                userflows.value = { flows: [] };
            }
            if (!commandHistory.value || !commandHistory.value?.history) {
                commandHistory.value = { history: [] };
            }

            let output = outpu.map((o) => o.stdout);
            // parse the output of nest caddy list
            let domains = output[1];
            let parsed: [string, string][] = [];
            for (let domain of domains.match(/- (.*) \((.*)\)/gm)) {
                let part = domain.matchAll(/- (.*) \((.*)\)/gm).next();
                if (part.value?.at(1) && part.value?.at(2)) {
                    parsed.push([part.value[1], part.value[2]]);
                    self(`Parsed domain ${part.value[1]} at socket ${part.value[2]}`);
                } else {
                    self(`Failed to parse domain ${domain}. An error occured.`);
                }
            }

            let caddyfile = output[2];
            if (!caddyfile) {
                self('Caddyfile was empty. Double check please.');
            }

            let caddyJSON = attempt(() => JSON.parse(output[3]));
            let parsedCaddy: any = {};
            if (caddyJSON) {
                if (caddyJSON.admin) {
                    parsedCaddy.admin = true;
                }
                if (caddyJSON.apps?.http?.servers) {
                    parsedCaddy.servers = Object.entries(caddyJSON.apps.http.servers).length;
                }
            }

            caddyStore.set = true;
            // noinspection JSValidateTypes
            caddyStore.value = {
                caddyfile: String(caddyfile),
                domains: parsed,
                json: parsedCaddy,
                lastUpdated: new Date(),
            };
            self('Caddyfile and domains saved successfully.');

            // nest resources parsing
            let resources = output[4];
            let disk = resources.matchAll(/^Disk usage: (\d*\.\d*).*used out of (\d*\.\d*).*limit$/gm).next();
            let memory = resources.matchAll(/^Memory usage: (\d*\.\d*).*used out of (\d*\.\d*).*limit$/gm).next();
            let diskLimit: [number, number] = [parseFloat(disk.value[1]), parseFloat(disk.value[2])];
            let memoryLimit: [number, number] = [parseFloat(memory.value[1]), parseFloat(memory.value[2])];
            if (!(diskLimit[0] && diskLimit[1])) {
                self('seems there was an issue parsing disk usage.');
            }
            if (!(memoryLimit[0] && memoryLimit[1])) {
                self('seems there was an issue parsing memory usage.');
            }

            let [users, since] = output[5].split('--separate--');
            let userCount = parseInt(users.matchAll(/,\s*?(\d+) users,/gm).next().value[1]);
            let sinceDate = new Date(since.trim());
            serverStore.set = true;
            serverStore.value = {
                diskUsage: diskLimit,
                memoryUsage: memoryLimit,
                users: userCount,
                runningSince: sinceDate,
                lastUpdated: new Date(),
            };

            let services = output[6];
            let servicesPreprocess = services.matchAll(/[●×] (.+?\.service) ?-? ?(.+?)?\s.+Loaded: (.*) \((.+?); (.+?); preset: (.+?)\)\s+Active: (.*?) \((.*)\) since (.*?);.*\s.*?Main PID: (\d+) \((.*)\)/gm);

            let service: IteratorResult<RegExpExecArray, any>;
            servicesStore.set = true;
            servicesStore.value = {
                services: [],
                lastUpdated: new Date(),
            };
            while (((service = servicesPreprocess.next()), !service.done)) {
                let [_, name, description, loaded, path, enabled, preset, active, status, since, pid, exec] = service.value;
                servicesStore.value.services.push({
                    name,
                    description: description || null,
                    loaded: loaded as 'loaded',
                    path,
                    enabled: enabled as 'enabled' | 'disabled',
                    preset: preset as 'enabled' | 'disabled',
                    active: active as 'active' | 'inactive' | 'failed',
                    status,
                    since: new Date(since),
                    pid: parseInt(pid),
                    exec,
                });
            }

            await saveAll();
        },
        task: 'Finalising setup',
    },
];

const saveFiles = (files: string[], backup: boolean): Task[] => {
    let tasks: Task[] = [];

    for (let file of files) {
        const backupCommand = `mv ${file} ${file}.bak`;

        if (filesystem.value.fileData[file].deletedFile) {
            tasks.push({
                command: `rm ${file}`,
                task: `Deleting file ${file}`,
            });
            continue;
        }
        if (backup && !filesystem.value.fileData[file].newFile)
            tasks.push({
                command: backupCommand,
                task: `Backing up original file ${file} to ${file}.bak`,
            });
    }

    for (let file of files) {
        if (filesystem.value.fileData[file].deletedFile) continue;
        tasks.push({
            command: null,
            task: `Saving file ${file} to server`,
            frontend: true,
            promise: async (_, self) => {
                await EditFile(file, filesystem.value.fileData[file].modified);
                await saveAll();
            },
        });
    }

    return tasks;
};

export default {
    startup,
    saveFiles,
};
