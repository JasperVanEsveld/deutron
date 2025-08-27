# Deutron API

It is recommended to use a library for your language.
If there is no library you can use this document to create your own, use `--verbose` to show the messages that are send between deutron and your backend.
Deutron uses stringified JSON messages over stdin and stdout.
This document shows examples all the messages that can be send and received.

## Backend - stdout

Backend can use stdout to do the following: create a window, control a window, message a window or request info.
These message all need to be preceded by `DEUTRON_IPC:` followed by a JSON stringified command.

### Create window

```ts
{
    "Window": {
        //Enables/disables devtools
        dev_tools: boolean,

        // Title of the window
        title: string,

        // Path to html file
        url: string,

        // Window dimensions
        width: number,
        height: number,
        
        // Allow for transparent background
        transparent: boolean, 

        // Disables the default window decorations
        // Including: borders, minimize, maximize close, etc.
        no_decorations: boolean, 
    }
}

```

### Control

```ts
{
    "Control":{
        // The target window
        target: number,
        
        // The action to perform
        control: "Maximize" | "Minimize" | "Close" | "Drag"
    }
}
```

### Message

```ts
{
    "Control":{
        // The target window
        target: number,
        
        // The action to perform
        control: "Maximize" | "Minimize" | "Close" | "Drag"
    }
}
```

### Requests

Requests are responded to by deutron with an Info message later, see below.
There are two requests you can send:
```ts
{
    "Request": {
        "Window": number
    }
}
```
and
```ts
{
    "Request": {
        "Windows": null
    }
}
```

## Backend - stdin

Deutron sends messages, information, responses and errors over stdin.
These are NOT preceded by `DEUTRON_IPC:`.

### Window lifetime events:
```ts
{
    "Info":{
        "Created": number // Window ID
    }
}
```
```ts
{
    "Info":{
        "Loaded": number // Window ID
    }
}
```
```ts
{
    "Info":{
        "Closed": number // Window ID
    }
}
```

### Message

```ts
{
    "Message":{
        "from": number, // Window ID
        "data": string
    }
}
```

### Response

```ts
{
    "Info":{
        "Response": {
            "Window": {
                id: number;
                title: string;
                url: string;
                fullscreen: boolean;
            }
        }
    }
}
```

```ts
{
    "Info":{
        "Response": {
            "Windows": {
                id: number;
                title: string;
                url: string;
                fullscreen: boolean;
            }[]
        }
    }
}
```

### Error

Passes the error when deutron failed to serve a requested file.

```ts
{
    "Info":{
        "Error": string
    }
}
```
