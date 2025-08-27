{
    class Deutron {
        #info = [];
        #message = [];
        id;

        constructor() {
            this.#handleDragables();
            window.onload = () => {
                this.#send("Loaded");
            };
        }

        onInfo(listener) {
            this.#info.push(listener);
            return () => {
                const i = this.#info.indexOf(listener);
                if (i < 0) {
                    return false;
                }
                this.#info.splice(i, 1);
                return true;
            };
        }

        onMessage(listener) {
            this.#message.push(listener);
            return () => {
                const i = this.#message.indexOf(listener);
                if (i < 0) {
                    return false;
                }
                this.#message.splice(i, 1);
                return true;
            };
        }

        messageBackend(data) {
            this.#send({ Message: { target: "Backend", data } });
        }

        messageWindow(target, data) {
            this.#send({ Message: { target: { Window: target }, data } });
        }

        create(options = {}) {
            this.#send({ Window: { Create: options } });
        }

        fullscreen(target = undefined) {
            this.#send({ Control: { target, control: "Fullscreen" } });
        }
        minimize(target = undefined) {
            this.#send({ Control: { target, control: "Minimize" } });
        }
        maximize(target = undefined) {
            this.#send({ Control: { target, control: "Maximize" } });
        }
        close(target = undefined) {
            this.#send({ Control: { target, control: "Close" } });
        }

        get_windows() {
            return this.#request("Windows");
        }

        get_window(target = undefined) {
            return this.#request("Window", target);
        }

        isFullscreen() {
            return this.get_window().then((info) => info.fullscreen);
        }

        #request(type, params = null) {
            return new Promise((resolve) => {
                const removeListener = this.onInfo((info) => {
                    if ("Response" in info) {
                        const response = info["Response"];
                        if (response === type) {
                            resolve();
                            removeListener();
                        } else if (type in response) {
                            resolve(response[type]);
                            removeListener();
                        }
                    }
                });
                this.#send({ Request: { [type]: params } });
            });
        }

        #send(message) {
            window.ipc.postMessage(JSON.stringify(message));
        }

        #handleDragables() {
            const isDragable = async (e) => {
                return (
                    e.target.classList.contains("drag-region") &&
                    (await window.deutron.isFullscreen()) == false
                );
            };
            // Listen to drag-region events
            document.addEventListener("mousedown", async (e) => {
                if (await isDragable(e)) {
                    e.detail === 2
                        ? window.deutron.maximize()
                        : this.#send({ Control: { control: "Drag" } });
                }
            });
            document.addEventListener("touchstart", async (e) => {
                if (await isDragable(e)) {
                    this.#send({ Control: { control: "Drag" } });
                }
            });
            this.id = new Promise((resolve) =>
                this.get_window().then((window) => {
                    resolve(window.id);
                    this.id = window.id;
                })
            );
        }

        // ! DON'T USE THIS METHOD !
        // only used in the Rust code to trigger events
        triggerEvent(event) {
            if ("Message" in event) {
                this.#message.forEach((listener) => listener(event.Message));
            } else {
                const { Info: message } = event;
                this.#info.forEach((listener) => listener(message));
            }
        }
    }
    window.deutron = new Deutron();
}
