<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import type { Browser } from "./defs";

    type Props = {
        urlToOpen?: string;
    };

    let { urlToOpen }: Props = $props();

    let openingBrowser = $state(false);

    let browsers = $derived(await invoke<Array<Browser>>("get_browsers"));

    const openBrowser = async (id: string) => {
        try {
            openingBrowser = true;
            await invoke<void>("open_url_in_browser", { url: urlToOpen, id: id });
        } finally {
            openingBrowser = false;
        }
    };
</script>

<div class="browser-grid">
    {#each browsers as browser}
        <div class="browser">
            <button>
                <!-- TODO: Add menu button to allow editing/removing this entry -->
            </button>
            <button
                onclick={() => openBrowser(browser.id)}
                class="browser-icon"
                disabled={openingBrowser}
            >
                <!-- TODO: Get icon using browser ID, and fallback to '🌐' emoji if none -->
                🌐
            </button>
            <p>{browser.name}</p>
        </div>
    {/each}
</div>

<style>
    .browser-grid {
        display: grid;
        grid-template-columns: repeat(128px, 4);
    }

    .browser-icon {
        font-size: 2.5rem;
        line-height: 1;
    }
</style>
