import * as monaco from 'monaco-editor';

// Import the workers in a production-safe way.
// This is different than in Monaco's documentation for Vite,
// but avoids a weird error ("Unexpected usage") at runtime
// @ts-ignore
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
// @ts-ignore
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
// @ts-ignore
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
// @ts-ignore
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
// @ts-ignore
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

self.MonacoEnvironment = {
    getWorker: function (_: string, label: string) {
        switch (label) {
            case 'json':
                return new jsonWorker();
            case 'css':
            case 'scss':
            case 'less':
                return new cssWorker();
            case 'html':
            case 'handlebars':
            case 'razor':
                return new htmlWorker();
            case 'typescript':
            case 'javascript':
                return new tsWorker();
            default:
                return new editorWorker();
        }
    }
};

const caddyfileLanguage = {
    defaultToken: '',
    tokenPostfix: '.caddyfile',

    keywords: [
        'import',
        'handle',
        'handle_path',
        'route',
        'respond',
        'redir',
        'reverse_proxy',
        'file_server',
        'log',
        'tls',
        'encode',
        'header',
        'request_uri',
        'uri',
        'rewrite',
        'not',
        'method',
        'host',
        'path',
        'expression',
        'basicauth',
        'root',
        'bind'
    ],

    brackets: [
        { open: '{', close: '}', token: 'delimiter.curly' },
    ],

    tokenizer: {
        root: [
            // Comments
            [/#.*$/, 'comment'],

            // Braces
            [/{/, { token: 'delimiter.curly', next: '@block' }],

            // Strings
            [/"([^"\\]|\\.)*$/, 'string.invalid'], // Non-terminated string
            [/"/, { token: 'string.quote', next: '@string' }],

            // Keywords
            [/@[a-zA-Z_]\w*/, 'keyword'],
            [
                /\b(?:import|handle|handle_path|route|respond|redir|reverse_proxy|file_server|log|tls|encode|header|request_uri|uri|rewrite|not|method|host|path|expression|basicauth|root|bind)\b/,
                'keyword',
            ],

            // IP addresses, ports, paths
            [/\b\d{1,3}(\.\d{1,3}){3}\b(:\d+)?/, 'number'],
            [/:?\d+/, 'number'],

            // Hostnames and paths
            [/\/[^\s{}"]*/, 'string.path'],
            [/[^\s{}"]+/, ''] // identifiers or literals
        ],

        block: [
            [/#.*$/, 'comment'],
            [/{/, { token: 'delimiter.curly', next: '@block' }],
            [/}/, { token: 'delimiter.curly', next: '@pop' }],
            [/"([^"\\]|\\.)*$/, 'string.invalid'],
            [/"/, { token: 'string.quote', next: '@string' }],
            [
                /\b(?:import|handle|handle_path|route|respond|redir|reverse_proxy|file_server|log|tls|encode|header|request_uri|uri|rewrite|not|method|host|path|expression|basicauth|root|bind)\b/,
                'keyword',
            ],
            [/:?\d+/, 'number'],
            [/\/[^\s{}"]*/, 'string.path'],
            [/[^\s{}"]+/, '']
        ],

        string: [
            [/[^\\"]+/, 'string'],
            [/\\./, 'string.escape'],
            [/"/, { token: 'string.quote', next: '@pop' }]
        ]
    }
};

monaco.languages.register({ id: 'caddyfile', extensions: [".caddyfile"], aliases: ["Caddyfile"] });
// @ts-ignore
monaco.languages.setMonarchTokensProvider('caddyfile', caddyfileLanguage);

export const supportedLanguages = () => {
    return monaco.languages.getLanguages().map(v => v.id)
}
export const highlighting = (filename: string) => {
    let languages = monaco.languages.getLanguages()
    for (let lang of languages) {
        if (lang.extensions && lang.extensions.some(ext => filename.endsWith(ext))) {
            return lang.id;
        }
        if (lang.aliases && lang.aliases.some(alias => filename.toLowerCase() === alias.toLowerCase())) {
            return lang.id;
        }
    }
    return 'plaintext';
}

export default monaco;