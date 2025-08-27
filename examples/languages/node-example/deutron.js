import * as readline from "readline";

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
});

const defaultOptions = {
    dev_tools: false,
    title: "WebView",
    url: "./index.html",
    no_decorations: false,
    transparent: false,
    width: 800,
    height: 600,
};

class EventEmitter {
    listeners = [];

    emit(...args) {
        this.listeners.forEach((listener) => listener(...args));
    }

    on(listener) {
        this.listeners.push(listener);
        return () => this.listeners.splice(this.listeners.indexOf(listener), 1);
    }
}

export class Deutron {
    #info = new EventEmitter();
    #message = new EventEmitter();
    #ready = new EventEmitter();
    dir;

    constructor(window) {
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

    #listenOutput() {
        rl.on("line", (line) => {
            const data = JSON.parse(line);
            if ("Ready" in data) {
                this.#ready.emit({ type: "Ready", dir: data.Ready });
            } else if ("Info" in data) {
                const type = Object.keys(data.Info)[0];
                this.#info.emit({ type, data: data.Info[type] });
            } else {
                this.#message.emit(data.Message);
            }
        });
    }

    #request(type, params = null) {
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

    getWindows() {
        return this.#request("Windows");
    }

    getWindow(id) {
        return this.#request("Window", id);
    }

    onInfo(callback) {
        return this.#info.on(callback);
    }

    onReady(callback) {
        return this.#ready.on(callback);
    }

    onMessage(callback) {
        return this.#message.on(callback);
    }

    #send(event) {
        console.log("DEUTRON_IPC:" + JSON.stringify(event));
    }

    #controlWindow(target, control) {
        this.#send({ Window: { Control: { target, control } } });
    }

    message(target, data) {
        this.#send({ Message: { target, data } });
    }

    close(target) {
        this.#controlWindow(target, "Close");
    }

    drag(target) {
        this.#controlWindow(target, "Drag");
    }

    minimize(target) {
        this.#controlWindow(target, "Minimize");
    }

    maximize(target) {
        this.#controlWindow(target, "Maximize");
    }

    fullscreen(target) {
        this.#controlWindow(target, "Fullscreen");
    }

    create(options) {
        const fullOptions = { ...defaultOptions, ...options };
        this.#send({ Window: { Create: fullOptions } });
    }
}
