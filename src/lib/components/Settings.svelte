<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    const openInVscode = async () => {
        await invoke<void>("open_config_in_vscode");
    };

    const reload = async () => {
        window.location.reload();
    };
</script>

<details>
    <summary>Settings</summary>
    <div>
        <button onclick={openInVscode}>Open Config in VS Code</button>
        <button onclick={reload}>Reload Config</button>
    </div>
</details>

<style>
</style>
