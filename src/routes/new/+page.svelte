<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";

    let name = $state("");
    let pathToExecutable = $state("");
    let iconFile = $state.raw<File | null>(null);

    const fileToBase64 = (file: File): Promise<string> => {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => {
                const result = reader.result as string;
                // Remove the data URL prefix (e.g., "data:image/png;base64,")
                const base64 = result.split(",")[1];
                resolve(base64);
            };
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });
    };

    const handleFileChange = (ev: Event) => {
        const input = ev.target as HTMLInputElement;
        iconFile = input.files?.[0] ?? null;
    };

    const addNew = async (ev: SubmitEvent) => {
        ev.preventDefault();

        if (name === "" || pathToExecutable === "") {
            // Basic validation. Will do something better later.
            return;
        }

        let icon: string | null = null;
        let iconMime: string | null = null;

        if (iconFile) {
            icon = await fileToBase64(iconFile);
            iconMime = iconFile.type;
        }

        await invoke<void>("add_new_browser", {
            name: name,
            path: pathToExecutable,
            icon: icon,
            iconMime: iconMime,
        });

        await goto("/");
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
    <input id="icon" type="file" accept="image/png, image/jpeg, image/webp, image/avif" onchange={handleFileChange} />
    <button type="submit">Create</button>
</form>
