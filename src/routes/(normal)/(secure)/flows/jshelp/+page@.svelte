<script>
</script>

<div class="p-2 font-sans">
    <div class="text-2xl font-bold">NestHelper: JavaScript Task docs</div>
    <div>JavaScript has a lot of uses to help test, save, or do whatever you want with outputs from other tasks.</div>
    <br />
    <div class="text-xl font-bold">Success and Error</div>
    <div>Success is triggered when the promise you write resolves. The return value you provide has no meaning. If you want to error out your task (for example, if a condition failed), throw an Error normally.</div>
    <br />
    <div class="text-xl font-bold">Accessing output of commands</div>
    <div>
        The output of commands is provided by the variable <b>output</b> in an array that contains the <b>stdout</b> and
        <b>stderr</b> of each command.
    </div>
    <div class="bg-gray-800 text-white p-2 rounded-lg shadow-md m-2">
        <pre class="overflow-x-auto"><code class="language-javascript"
                >output: {'{'}
    stdout: string,
    stderr: string
}[]</code
            ></pre>
    </div>
    <div>Indices are based on the placement of the task. The first task will have an index of zero and the second: 1, etc.</div>
    <br />
    <div class="text-xl font-bold">Outputting from a JavaScript task</div>
    <div>
        You can output to the Flow's output by using the provided functions <b>log</b> and <b>logError</b>. Do note that output from a JavaScript task is not accessible by any other task, and is only for visual inspection. 
    </div>
    <div class="bg-gray-800 text-white p-2 rounded-lg shadow-md m-2 overflow-x-auto">
        <pre class=""><code class="language-javascript">function log(output: string): void;</code></pre>
        <pre class=""><code class="language-javascript">function logError(output: string): void;</code></pre>
    </div>
    <br />
    <div class="text-xl font-bold">Fetching resources protected by CORS</div>
    <div>
        If you need to fetch resources or access content that could be protected by CORS, use the <b>fetchNoCors</b>
        function, which will make the request through the backend rather than through the JavaScript engine. This helper function is less featured than the native fetch, so use <b>window.fetch</b> when possible.
    </div>
    <div class="bg-gray-800 text-white p-2 rounded-lg shadow-md m-2">
        <pre class="overflow-x-auto"><code class="language-javascript"
                >async function fetchNoCors(url: string, options: {'{'}
    method?: string,
    headers?: {'{'} [header: string]: string },
    body?: string
} = {'{'}}): Promise&lt;{'{'}
    ok: boolean,
    status: number,
    headers: {'{'} [header: string]: string },
    text: () => Promise&lt;string&gt;,
    json: () => Promise&lt;any&gt;
}&gt;</code
            ></pre>
    </div>
</div>
