<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let name = $state("");
    let pathToExecutable = $state("");
    // TODO: Store icon using `$state.raw`

    const addNew = async (ev: SubmitEvent) => {
        ev.preventDefault();

        if (name === "" || pathToExecutable === "") {
            // Basic validation. Will do something better later.
            return;
        }

        await invoke<void>("add_new_browser", {
            name: name,
            path: pathToExecutable,
            // TODO: Also submit icon file
        });
    };
</script>

<header>
    <a href="/">Back</a>
</header>

<form onsubmit={addNew}>
    <label for="name">Name:</label>
    <input bind:value={name} id="name" type="text" />
    <label for="path-to-executable">Path to executable:</label>
    <input bind:value={pathToExecutable} id="path-to-executable" type="text" />
    <label for="icon">Icon:</label>
    <input id="icon" type="file" accept="image/png, image/jpeg, image/webp, image/avif" />
    <button type="submit">Create</button>
</form>
