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
    messageBackend(data: string): void;
    messageWindow(target: number, data: string): void;

    // Listen to events
    // Call the returned function to remove the listener
    onInfo(listener: (info: Info) => void): () => void;
    onMessage(listener: (message: Message) => void): () => void;
}
declare global {
    const deutron: Deutron;
}

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

type Message = { from: number; data: string };

type Info = Created | Loaded | Closed | DeutronResponse | Log | DeutronError;
type Created = { type: "Created"; data: number };
type Loaded = { type: "Loaded"; data: number };
type Closed = { type: "Closed"; data: number };
type DeutronResponse = { type: "Response"; data: Record<string, unknown> };
type Log = { type: "Log"; data: string };
type DeutronError = { type: "Error"; data: string };

export {};
