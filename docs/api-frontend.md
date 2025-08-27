# Deutron API

On the frontend side there is a global `deutron` object that has the following structure:

```ts
interface Deutron {
    id: number;

    // Create a new window
    create(options?: WindowConfig): void;

    // Basic controls
    // Assumes current window if no target provided
    close(target?: number): void;
    maximize(target?: number): void;
    minimize(target?: number): void;
    fullscreen(target?: number): void;

    // Gets window information
    get_window(target?: number): Promise<Window>;
    get_windows(): Promise<Window[]>;
    isFullscreen(): Promise<boolean>;

    // Send messages to the backend or another window
    messageBackend(data): void;
    messageWindow(id, data): void;

    // Listen to events
    // Call the returned function to remove the listener
    onInfo(listener: (info) => void): () => void;
    onMessage(listener: (message: { from; data }) => void): () => void;
}
```
With `WindowConfig` and `Window` being:
```ts
interface WindowConfig {
    dev_tools: boolean;
    title: string;
    url: string;
    width: 800;
    height: 600;
    transparent: boolean;
    no_decorations: boolean;
}

interface Window {
    id: number;
    title: string;
    url: string;
    fullscreen: boolean;
}
```
