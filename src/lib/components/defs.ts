export type Browser = {
    readonly id: string;
    name: string;
    path: string;
    icon: string | null; // File extension if icon exists
};

export type BrowserIcon = {
    data: string; // Base64-encoded image data
    mime_type: string;
};
