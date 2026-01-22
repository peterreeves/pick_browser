<script lang="ts">
    import Globe from "@lucide/svelte/icons/globe";
    import DefaultBrowser from "$lib/components/DefaultBrowser.svelte";
    import OpenUrl from "$lib/components/OpenUrl.svelte";
    import Settings from "$lib/components/Settings.svelte";
</script>

<main class="container">
    <header>
        <DefaultBrowser />
        <Settings />
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
