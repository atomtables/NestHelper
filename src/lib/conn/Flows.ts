import {caddy as caddyStore, save, server as serverStore, services as servicesStore} from "$lib/state/states.svelte.js";
import {type Task} from "$lib/conn/Workflow.svelte.js";
import {arrWithUpdate} from "$lib/state/states.svelte";

const caddy: Task[] = [
    { command: "nest caddy list", task: "Getting connected domains" },
    { command: "cat Caddyfile", task: "Getting Caddyfile" },
]

const server: Task[] = [
    { command: "nest resources", task: "Getting current resource usage" },
    { command: "uptime && echo --separate-- && uptime -s", task: "Getting server stats"},
]

const startup: Task[] = [
    { command: null, task: "Connecting via SSH" },
    ...caddy,
    ...server,
    { command: "for file in ~/.config/systemd/user/*; do if [ -f \"$file\" ]; then systemctl --user status \"${file#/home/atomtables/.config/systemd/user/}\" | grep -e Loaded -e Active -e ● -e × -e \"Main PID\"; fi; done", task: "Getting systemctl services" },
    { command: null, frontend: true, promise: (async (output, self) => {
            // parse the output of nest caddy list
            let domains = output[1];
            let parsed: [string, string][] = [];
            for (let domain of domains.match(/- (.*) \((.*)\)/gm)) {
                let part = domain.matchAll(/- (.*) \((.*)\)/gm).next()
                if (part.value?.at(1) && part.value?.at(2)) {
                    parsed.push([part.value[1], part.value[2]])
                    self(`Parsed domain ${part.value[1]} at socket ${part.value[2]}`)
                } else {
                    self(`Failed to parse domain ${domain}. An error occured.`);
                }
            }

            let caddyfile = output[2]
            if (!caddyfile) {
                self("Caddyfile was empty. Double check please.");
            }

            caddyStore.set = true;
            // noinspection JSValidateTypes
            caddyStore.value = {
                caddyfile: String(caddyfile),
                domains: parsed,
                lastUpdated: new Date()
            }
            await save(caddy);
            self("Caddyfile and domains saved successfully.");

            // nest resources parsing
            let resources = output[3];
            let disk = resources.matchAll(/^Disk usage: (\d*\.\d*).*used out of (\d*\.\d*).*limit$/gm).next()
            let memory = resources.matchAll(/^Memory usage: (\d*\.\d*).*used out of (\d*\.\d*).*limit$/gm).next();
            let diskLimit: [number, number] = [parseFloat(disk.value[1]), parseFloat(disk.value[2])]
            let memoryLimit: [number, number] = [parseFloat(memory.value[1]), parseFloat(memory.value[2])];
            if (!(diskLimit[0] && diskLimit[1])) {
                self("seems there was an issue parsing disk usage.")
            }
            if (!(memoryLimit[0] && memoryLimit[1])) {
                self("seems there was an issue parsing memory usage.")
            }

            let [users, since] = output[4].split("--separate--");
            let userCount = parseInt(users.matchAll(/, (\d+) users,/gm).next().value[1])
            let sinceDate = new Date(since.trim());
            serverStore.set = true;
            serverStore.value = {
                diskUsage: diskLimit,
                memoryUsage: memoryLimit,
                users: userCount,
                runningSince: sinceDate,
                lastUpdated: new Date()
            }
            await save(server);

            let services = output[5]
            let servicesPreprocess = services.matchAll(/[●×] (.+?\.service) ?-? ?(.+?)?\s.+Loaded: (.*) \((.+?); (.+?); preset: (.+?)\)\s+Active: (.*?) \((.*)\) since (.*?);.*\s.*?Main PID: (\d+) \((.*)\)/gm)

            let service: IteratorResult<RegExpExecArray, any>;
            servicesStore.set = true;
            servicesStore.value = arrWithUpdate([]); // since we're loading the latest information
            while (service = servicesPreprocess.next(), !service.done) {
                let [_, name, description, loaded, path, enabled, preset, active, status, since, pid, exec] = service.value;
                servicesStore.value.push({
                    name,
                    description: description || null,
                    loaded: loaded as "loaded",
                    path,
                    enabled: enabled as "enabled" | "disabled",
                    preset: preset as "enabled" | "disabled",
                    active: active as "active" | "inactive" | "failed",
                    status,
                    since: new Date(since),
                    pid: parseInt(pid),
                    exec
                });
            }
            self(`Services loaded successfully.: ${servicesStore.value[0].name}`);
            await save(servicesStore);
        }), task: "Finalising setup" },
]

export default {
    startup
}