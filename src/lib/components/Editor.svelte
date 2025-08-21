<!-- credits to ala-garbaa-pro for the monaco svelte integration -->
<script lang="ts">
    import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
    import { onDestroy, onMount } from 'svelte';

    let editor: Monaco.editor.IStandaloneCodeEditor;
    let monaco: typeof Monaco;
    let editorContainer: HTMLElement;

    let { value = $bindable(), language = 'html', theme = 'vs-dark', class: className } = $props();

    onMount(async () => {
        monaco = (await import('$lib/helpers/monaco')).default
        console.log(monaco.languages.getLanguages())

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
            if (e.isFlush) {
                /* editor.setValue(value); */
            } else {
                value = editor?.getValue() ?? ' ';
            }
        });
    });

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
        monaco?.editor.getModels().forEach((model) => model.dispose());
        editor?.dispose();
    });
</script>

<div class="h-full w-full {className}" bind:this={editorContainer}></div>