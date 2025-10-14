<script>
    let {size = 40, strokeWidth = 4, type = "primary", class: className} = $props();
</script>

<style>
    .spinner {
        animation: rotate 1.2s linear infinite;
        transform-origin: center;
    }

    @keyframes rotate {
        0%   { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }

    .circle {
        stroke-dasharray: 80;
        stroke-dashoffset: 60;
        stroke-linecap: round;
        animation: dash 1.4s ease-in-out infinite;
    }

    @keyframes dash {
        0% {
            stroke-dasharray: 1, 200;
            stroke-dashoffset: 0;
        }
        50% {
            stroke-dasharray: 90, 150;
            stroke-dashoffset: -35;
        }
        100% {
            stroke-dasharray: 90, 150;
            stroke-dashoffset: -124;
        }
    }
</style>

<svg
        class="spinner {className}"
        width={size}
        height={size}
        viewBox="0 0 50 50"
>
    <defs>
        <linearGradient id="primary" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#5a00c8" />
            <stop offset="100%" stop-color="#dd17c6" />
        </linearGradient>
        <linearGradient id="secondary" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="#00d2ff" />
            <stop offset="100%" stop-color="#4a77f5" />
        </linearGradient>
    </defs>

    <circle
            class="circle"
            stroke={type === "primary" ? "url(#primary)" : "url(#secondary)"}
            cx="25"
            cy="25"
            r="20"
            fill="none"
            stroke-width={strokeWidth}
    />
</svg>