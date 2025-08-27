import sys
import json


defaultOptions = {
    "dev_tools": False,
    "title": "WebView",
    "url": "./index.html",
    "no_decorations": False,
    "transparent": False,
    "width": 800,
    "height": 600,
}


class Deutron:
    def __init__(self, window=None):
        self.stdin = StdIn()
        self.messageEmitter = EventEmitter()
        self.readyEmitter = EventEmitter()
        self.infoEmitter = EventEmitter()
        self.stdin.add_listener(self.__handle_in)
        if window != None:
            self.create(window)

    def start(self):
        self.stdin.start()

    def on_ready(self, callback):
        return self.readyEmitter.add_listener(callback)

    def on_message(self, callback):
        return self.messageEmitter.add_listener(callback)

    def on_info(self, callback):
        return self.infoEmitter.add_listener(callback)

    def __handle_in(self, message):
        try:
            data = json.loads(message)
        except:
            data = message

        if "Ready" in data:
            self.readyEmitter.emit({"type": "Ready", "data": data["Ready"]})
        elif "Info" in data:
            type = list(data["Info"])[0]
            self.infoEmitter.emit({"type": type, "data": data["Info"][type]})
        else:
            self.messageEmitter.emit(data["Message"])

    def __request(self, type, callback, params=None):
        removeListener = None

        def handle_request(event):
            if event.type == "Response":
                response = event.data
                if (type in response):
                    data = response[type]
                    callback(data)
                    print(removeListener)
                    removeListener()
        removeListener = self.on_info(handle_request)
        request = {"Request": {}}
        request["Request"][type] = params
        self.send(request)

    def get_windows(self, callback):
        return self.__request("Windows", callback, params=None)

    def create(self, options={}):
        fullOptions = defaultOptions | options
        self.send({"Window": {"Create": fullOptions}})

    def get_window(self, target):
        self.request("Window", target)

    def control_window(self, target, control):
        self.send({"Window": {"Control": {"target": target, "control": control}}})

    def message(self, target, message):
        self.send({"Message": {"target": target, "data": message}})

    def close(self, target):
        self.control_window(target, "Close")

    def minimize(self, target):
        self.control_window(target, "Minimize")

    def maximize(self, target):
        self.control_window(target, "Maximize")

    def fullscreen(self, target):
        self.control_window(target, "Fullscreen")

    def send(self, event):
        try:
            message = "DEUTRON_IPC:" + json.dumps(event) + "\n"
            sys.stdout.write(message)
        except:
            print("Failed to send: ")
            print(event)


class StdIn:
    def __init__(self):
        self.emitter = EventEmitter()

    def start(self):
        while True:
            self.emitter.emit(input())

    def add_listener(self, callback):
        self.emitter.add_listener(callback)


class EventEmitter:
    def __init__(self):
        self.listeners = []

    def add_listener(self, callback):
        self.listeners.append(callback)

        return lambda: self.remove_listener(callback)

    def remove_listener(self, callback):
        try:
            self.listeners.remove(callback)
        except ValueError:
            pass

    def emit(self, data):
        for callback in self.listeners:
            print(data)
            callback(data)
