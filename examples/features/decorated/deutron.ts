const defaultOptions: WindowOptions = {
    dev_tools: false,
    title: "WebView",
    url: "./index.html",
    no_decorations: false,
    transparent: false,
    width: 800,
    height: 600,
};

export class Deutron {
    #info = new EventEmitter<[Info]>();
    #message = new EventEmitter<[Message]>();
    #ready = new EventEmitter<[Ready]>();
    dir;

    constructor(window: Partial<WindowOptions>) {
        this.#listenOutput();
        this.dir = new Promise((resolve) =>
            this.onReady((data) => {
                return resolve(data.dir);
            })
        );
        if (window != undefined) {
            this.create(window);
        }
    }

    async #listenOutput() {
        for await (const line of Deno.stdin.readable.pipeThrough(
            new TextDecoderStream()
        )) {
            const data = JSON.parse(line);
            if ("Ready" in data) {
                this.#ready.emit({ type: "Ready", dir: data.Ready });
            } else if ("Info" in data) {
                const type = Object.keys(data.Info)[0];
                this.#info.emit({ type: type as any, data: data.Info[type] });
            } else {
                this.#message.emit(data.Message);
            }
        }
    }

    #request(type: string, params: any = null) {
        return new Promise((resolve) => {
            const removeListener = this.onInfo((event) => {
                if (event.type == "Response") {
                    const response = event.data;
                    if (type in response) {
                        const data = response[type];
                        resolve(data);
                        removeListener();
                    }
                }
            });
            this.#send({ Request: { [type]: params } });
        });
    }

    getWindows(): Promise<WindowInfo[]> {
        return this.#request("Windows") as any;
    }

    getWindow(target: number): Promise<WindowInfo> {
        return this.#request("Window", target) as any;
    }

    onInfo(callback: Listener<[Info]>) {
        return this.#info.on(callback);
    }

    onReady(callback: Listener<[Ready]>) {
        return this.#ready.on(callback);
    }

    onMessage(callback: Listener<[Message]>) {
        return this.#message.on(callback);
    }

    #send(event: BackendEvent) {
        console.log("DEUTRON_IPC:" + JSON.stringify(event));
    }

    #controlWindow(target: number, control: WindowControl) {
        this.#send({ Window: { Control: { target, control } } });
    }

    message(target: number, data: string) {
        this.#send({ Message: { target, data } });
    }

    close(target: number) {
        this.#controlWindow(target, "Close");
    }

    drag(target: number) {
        this.#controlWindow(target, "Drag");
    }

    minimize(target: number) {
        this.#controlWindow(target, "Minimize");
    }

    maximize(target: number) {
        this.#controlWindow(target, "Maximize");
    }

    fullscreen(target: number) {
        this.#controlWindow(target, "Fullscreen");
    }

    create(options: Partial<WindowOptions>) {
        const fullOptions = { ...defaultOptions, ...options };
        this.#send({ Window: { Create: fullOptions } });
    }
}

type Listener<T extends any[]> = (...args: T) => void;
class EventEmitter<T extends any[]> {
    listeners: Listener<T>[] = [];

    emit(...args: T) {
        this.listeners.forEach((listener) => listener(...args));
    }

    on(listener: Listener<T>) {
        this.listeners.push(listener);
        return () => this.listeners.splice(this.listeners.indexOf(listener), 1);
    }
}

/**
 * Types
 */
type Message = { from: number; data: string };
type Info = Created | Loaded | Closed | Response | Log | Error;
type Created = { type: "Created"; data: number };
type Loaded = { type: "Loaded"; data: number };
type Closed = { type: "Closed"; data: number };
type Response = { type: "Response"; data: Record<string, unknown> };
type Log = { type: "Log"; data: string };
type Error = { type: "Error"; data: string };
type Ready = { type: "Ready"; dir: string };
type WindowInfo = {
    id: number;
    title: string;
    url: string;
    fullscreen: boolean;
};
type WindowOptions = {
    dev_tools: boolean;
    title: string;
    url: string;
    icon?: string;
    no_decorations: boolean;
    transparent: boolean;
    width: number;
    height: number;
};
type WindowControl = "Fullscreen" | "Maximize" | "Minimize" | "Drag" | "Close";
type MessageEvent = { target: number; data: unknown };
type WindowEvent =
    | { Create: WindowOptions }
    | { Control: { target: number; control: WindowControl } };
type RequestEvent = Record<string, unknown>;
type BackendEvent =
    | { Message: MessageEvent }
    | { Window: WindowEvent }
    | { Request: RequestEvent };
