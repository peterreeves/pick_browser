<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import ArrowLeft from "@lucide/svelte/icons/arrow-left";
    import Upload from "@lucide/svelte/icons/upload";
    import Image from "@lucide/svelte/icons/image";
    import X from "@lucide/svelte/icons/x";

    let name = $state("");
    let pathToExecutable = $state("");
    let iconFile = $state.raw<File | null>(null);
    let iconPreview = $state<string | null>(null);
    let submitting = $state(false);

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
        const file = input.files?.[0] ?? null;
        iconFile = file;

        if (file) {
            const reader = new FileReader();
            reader.onload = () => {
                iconPreview = reader.result as string;
            };
            reader.readAsDataURL(file);
        } else {
            iconPreview = null;
        }
    };

    const clearIcon = () => {
        iconFile = null;
        iconPreview = null;
    };

    const addNew = async (ev: SubmitEvent) => {
        ev.preventDefault();

        if (name === "" || pathToExecutable === "") {
            return;
        }

        submitting = true;

        try {
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
        } finally {
            submitting = false;
        }
    };
</script>

<main class="container">
    <header>
        <a href="/" class="back-link">
            <ArrowLeft size={20} />
            <span>Back</span>
        </a>
    </header>

    <h1>Add Browser</h1>

    <form onsubmit={addNew}>
        <div class="form-group">
            <label for="name">Name</label>
            <input
                bind:value={name}
                id="name"
                type="text"
                placeholder="e.g. Chrome, Firefox"
                required
            />
        </div>

        <div class="form-group">
            <label for="path-to-executable">Path to executable</label>
            <input
                bind:value={pathToExecutable}
                id="path-to-executable"
                type="text"
                placeholder="e.g. C:\Program Files\..."
                required
            />
        </div>

        <div class="form-group">
            <label for="icon">Icon (optional)</label>
            {#if iconPreview}
                <div class="icon-preview">
                    <img src={iconPreview} alt="Icon preview" />
                    <button
                        type="button"
                        class="btn btn-ghost btn-icon remove-icon"
                        onclick={clearIcon}
                    >
                        <X size={16} />
                    </button>
                </div>
            {:else}
                <label class="file-upload" for="icon">
                    <Upload size={24} />
                    <span>Choose image</span>
                    <span class="file-hint">PNG, JPEG, WebP, or AVIF</span>
                </label>
            {/if}
            <input
                id="icon"
                type="file"
                accept="image/png, image/jpeg, image/webp, image/avif"
                onchange={handleFileChange}
                class="file-input"
            />
        </div>

        <div class="form-actions">
            <a href="/" class="btn">Cancel</a>
            <button
                type="submit"
                class="btn btn-primary"
                disabled={submitting || !name || !pathToExecutable}
            >
                {#if submitting}
                    Adding...
                {:else}
                    Add Browser
                {/if}
            </button>
        </div>
    </form>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        padding: 1.5rem;
        max-width: 480px;
        margin: 0 auto;
    }

    header {
        display: flex;
        align-items: center;
    }

    .back-link {
        display: inline-flex;
        align-items: center;
        gap: 0.375rem;
        color: var(--text-secondary);
        text-decoration: none;
        font-size: 0.875rem;
        font-weight: 500;
        transition: color 0.15s ease;
    }

    .back-link:hover {
        color: var(--text-primary);
        text-decoration: none;
    }

    h1 {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--text-primary);
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .form-group label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--text-primary);
    }

    .form-group input[type="text"] {
        width: 100%;
    }

    .file-input {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border: 0;
    }

    .file-upload {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        padding: 1.5rem;
        border: 2px dashed var(--border-color);
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
        color: var(--text-secondary);
        cursor: pointer;
        transition:
            border-color 0.15s ease,
            background-color 0.15s ease;
    }

    .file-upload:hover {
        border-color: var(--zinc-400);
        background-color: var(--bg-tertiary);
    }

    .file-upload span {
        font-size: 0.875rem;
        font-weight: 500;
    }

    .file-hint {
        font-size: 0.75rem !important;
        font-weight: 400 !important;
        color: var(--text-muted);
    }

    .icon-preview {
        position: relative;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 80px;
        height: 80px;
        border: 1px solid var(--border-color);
        border-radius: 0.5rem;
        background-color: var(--bg-secondary);
        overflow: hidden;
    }

    .icon-preview img {
        width: 100%;
        height: 100%;
        object-fit: contain;
    }

    .remove-icon {
        position: absolute;
        top: 0.25rem;
        right: 0.25rem;
        padding: 0.25rem;
        background-color: var(--bg-primary);
        border-radius: 50%;
    }

    .form-actions {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        padding-top: 0.5rem;
    }
</style>
