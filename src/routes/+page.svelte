<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    interface Browser {
        name: string;
        path: string;
        icon?: string;
    }

    let browsers = $state<Browser[]>([]);
    let configPath = $state<string>("");
    let url = $state<string>("https://example.com");
    let loading = $state(true);
    let error = $state<string | null>(null);

    onMount(async () => {
        try {
            const [browsersData, pathData] = await Promise.all([
                invoke<Browser[]>("get_browsers"),
                invoke<string>("get_config_path"),
            ]);
            browsers = browsersData;
            configPath = pathData;
        } catch (e) {
            error = e as string;
            console.error("Failed to load browsers:", e);
        } finally {
            loading = false;
        }
    });

    function selectBrowser(browser: Browser) {
        console.log(`Opening ${url} in ${browser.name}`);
        // TODO: Implement browser launching
    }
</script>

<main class="container">
    <h1>Pick a Browser</h1>

    {#if loading}
        <p class="loading">Loading browsers...</p>
    {:else if error}
        <div class="error">
            <p>Error loading configuration:</p>
            <pre>{error}</pre>
        </div>
    {:else}
        <div class="url-display">
            <strong>URL:</strong>
            <span class="url">{url}</span>
        </div>

        <div class="browser-list">
            {#each browsers as browser}
                <button class="browser-card" onclick={() => selectBrowser(browser)}>
                    <div class="browser-icon">🌐</div>
                    <div class="browser-name">{browser.name}</div>
                </button>
            {/each}
        </div>

        <div class="config-info">
            <p class="config-label">Config file location:</p>
            <code class="config-path">{configPath}</code>
        </div>
    {/if}
</main>

<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial,
            sans-serif;
        font-size: 14px;
        line-height: 1.5;
        font-weight: 400;

        color: #2c3e50;
        background-color: #f8f9fa;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    .container {
        padding: 20px;
        max-width: 500px;
        margin: 0 auto;
    }

    h1 {
        font-size: 1.5rem;
        margin: 0 0 20px 0;
        font-weight: 600;
        text-align: center;
    }

    .loading,
    .error {
        text-align: center;
        padding: 20px;
    }

    .error {
        color: #e74c3c;
    }

    .error pre {
        background: #fff;
        padding: 10px;
        border-radius: 6px;
        text-align: left;
        overflow-x: auto;
        font-size: 0.85rem;
    }

    .url-display {
        background: #fff;
        padding: 12px;
        border-radius: 8px;
        margin-bottom: 20px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    }

    .url-display strong {
        display: block;
        margin-bottom: 6px;
        font-size: 0.85rem;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: #7f8c8d;
    }

    .url {
        display: block;
        word-break: break-all;
        color: #3498db;
        font-family: "SF Mono", Monaco, Consolas, monospace;
        font-size: 0.9rem;
    }

    .browser-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
        gap: 12px;
        margin-bottom: 20px;
    }

    .browser-card {
        background: #fff;
        border: 2px solid #e0e0e0;
        border-radius: 10px;
        padding: 20px 10px;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
    }

    .browser-card:hover {
        border-color: #3498db;
        box-shadow: 0 4px 12px rgba(52, 152, 219, 0.15);
        transform: translateY(-2px);
    }

    .browser-card:active {
        transform: translateY(0);
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    }

    .browser-icon {
        font-size: 2.5rem;
        line-height: 1;
    }

    .browser-name {
        font-weight: 500;
        font-size: 0.95rem;
        color: #2c3e50;
        text-align: center;
    }

    .config-info {
        background: #fff;
        padding: 12px;
        border-radius: 8px;
        border: 1px solid #e0e0e0;
    }

    .config-label {
        font-size: 0.75rem;
        color: #7f8c8d;
        margin: 0 0 6px 0;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .config-path {
        display: block;
        font-family: "SF Mono", Monaco, Consolas, monospace;
        font-size: 0.8rem;
        color: #555;
        word-break: break-all;
        background: #f8f9fa;
        padding: 8px;
        border-radius: 4px;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            color: #ecf0f1;
            background-color: #1e1e1e;
        }

        .url-display,
        .browser-card,
        .config-info {
            background-color: #2d2d2d;
            border-color: #404040;
        }

        .error pre {
            background-color: #2d2d2d;
        }

        .url {
            color: #5dade2;
        }

        .browser-name {
            color: #ecf0f1;
        }

        .browser-card:hover {
            border-color: #5dade2;
            box-shadow: 0 4px 12px rgba(93, 173, 226, 0.2);
        }

        .config-path {
            background-color: #1e1e1e;
            color: #b0b0b0;
        }
    }
</style>
