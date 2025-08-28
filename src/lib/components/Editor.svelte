<!-- credits to ala-garbaa-pro for the monaco svelte integration -->
<script lang="ts">
    import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
    import { onDestroy, onMount } from 'svelte';
    import monaco from '$lib/helpers/monaco';

    let editor: Monaco.editor.IStandaloneCodeEditor;
    let editorContainer: HTMLElement;

    let { value = $bindable(), language = $bindable(), theme = 'vs-dark', class: className } = $props();

    onMount(async () => {
        try {
            // Your monaco instance is ready, let's display some code!
            editor = monaco.editor.create(editorContainer, {
                value,
                language,
                theme,
                automaticLayout: true,
                overviewRulerLanes: 0,
                overviewRulerBorder: true,
            });

            editor.onDidChangeModelContent((e) => {
                value = editor?.getValue() ?? ' ';
            });
        } catch (e) {
            console.error('Error mounting editor:', e);
        }
    });

    $effect(() => {
        try {
            if (monaco?.editor) {
                monaco.editor.setModelLanguage(editor.getModel(), language);
            }
        } catch (e) {
            console.error("Major ERROR: ", e)
        }
    })

    $effect(() => {
        if (value) {
            if (editor) {
                // check if the editor is focused
                if (editor.hasWidgetFocus()) {
                    // let the user edit with no interference
                } else {
                    if (editor?.getValue() ?? ' ' !== value) {
                        editor?.setValue(value);
                    }
                }
            }
        }
        if (value === '') {
            editor?.setValue(' ');
        }
    });

    onDestroy(() => {
        try {
            monaco?.editor.getModels().forEach((model) => model.dispose());
            editor?.dispose();
        } catch {
            console.error("failed to destroy")
        }
    });
</script>

<div class="h-full w-full {className}" bind:this={editorContainer}></div>