<script lang="ts">
    import DefaultBrowser from "$lib/components/DefaultBrowser.svelte";
    import OpenUrl from "$lib/components/OpenUrl.svelte";
    import Settings from "$lib/components/Settings.svelte";
</script>

<main class="container">
    <header>
        <DefaultBrowser></DefaultBrowser>
        <Settings></Settings>
    </header>

    <h1>Pick Browser</h1>

    <OpenUrl></OpenUrl>
</main>

<style>
    .container {
        padding: 24px;
        max-width: 512px;
        margin: 0 auto;
    }

    header {
        display: flex;
        justify-content: space-between;
    }

    h1 {
        font-size: 1.5rem;
        font-weight: 600;
        text-align: center;
    }

    @media (prefers-color-scheme: dark) {
    }
</style>
