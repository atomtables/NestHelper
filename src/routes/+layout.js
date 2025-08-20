import {auth} from "$lib/state/states.svelte.js";
import {goto} from "$app/navigation";
import {redirect} from "@sveltejs/kit";

export const load = ({ url }) => {
    if (!auth?.value?.username && !(url.pathname === '/' || url.pathname === "/onboarding")) {
        redirect(301, "/")
    }
}