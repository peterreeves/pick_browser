<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Globe from "@lucide/svelte/icons/globe";
    import X from "@lucide/svelte/icons/x";
    import DefaultBrowser from "$lib/components/DefaultBrowser.svelte";
    import OpenUrl from "$lib/components/OpenUrl.svelte";
    import Settings from "$lib/components/Settings.svelte";

    const closeApp = async () => {
        await invoke("exit_app");
    };
</script>

<main class="container">
    <header>
        <DefaultBrowser />
        <div class="header-actions">
            <Settings />
            <button class="btn btn-ghost btn-icon close-btn" onclick={closeApp} aria-label="Close">
                <X size={20} />
            </button>
        </div>
    </header>

    <div class="title">
        <Globe size={28} strokeWidth={2.5} />
        <h1>Pick Browser</h1>
    </div>

    <OpenUrl />
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        padding: 1.5rem;
        max-width: 580px;
        margin: 0 auto;
        min-height: 100%;
    }

    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .header-actions {
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    .close-btn {
        color: var(--text-muted);
    }

    .close-btn:hover {
        color: var(--red-600);
        background-color: var(--red-50);
    }

    .title {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        color: var(--text-primary);
    }

    h1 {
        font-size: 1.5rem;
        font-weight: 700;
        text-align: center;
        color: var(--text-primary);
    }
</style>
