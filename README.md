# Deutron

Command line tool thats adds a webview UI to your programs.
Uses std-in/out for communication, allowing it to easily be used by any programming language that can print and read user input.

## Usage


To create your first application just call `deutron` followed by your backend command.
```shell
deutron your backend command

--Examples--
deutron python ./app.py
deutron deno ./app.ts
```

This will create an executable that includes all files in the current directry.
If your backend relies on a runtime, you could add the runtime to the directory making the result portable.

For quicker iterations during development you can use `--debug`, which skips creating an executable and directly runs instead.
```shell
deutron --debug your backend command
```
Your backend can create a window with a simple log statement:
```ts
console.log(
    'DEUTRON_IPC:{"Window":{"Create":{"title":"Example","url":"./index.html"}}}'
);
```
Writing a wrapper library is recommended, check the examples for inspiration.
Want to add an example? Feel free to open a PR!

## Why?

Creating decent CLI tools is painless in many languages, adding a GUI is not.
Deutron aims to be a language agnostic way to add a GUI.
Web based desktop apps have existed for a while now, mainly Electron and (recently) Tauri.
Deutron lifts all the window and webview creation from your backend.
Your backend only requires support for stdin/stdout, no configs or specific file structure needed.

Making it a simple step to upgrade your CLI tool to an actual application.

## How?

Deutron launches your command and uses it's stdin/stdout to create, control and communicate with webview windows.
Internally it uses crates from Tauri, namely tao (crossplatform window manager) and wry (cross platform webview library).
Communication between your backend and deutron is done through json messages preceded by `DEUTRON_IPC:`, to see these messages use the `--verbose` flag.
Prints that are not preceded with `DEUTRON_IPC:` are passed on back to the terminal and printed normally.

When compiling it includes all files at the `--include` path in the final binary.
The binary unpacks these files the first time it is ran to a temp directory named: `binaryname_version_backend`.
Locations vary per operating system, for windows it in `AppData/Local/Temp/`.
Given no version change the binary is only unpacked once, unless compiled with `--no-cache`.

## Docs

For the frontend check [here](docs/api-frontend.md) and to see the raw backend messages check [here](docs/api-backend.md).

There are examples for Deno, Node.js and Python in the `examples` directory.
Other examples include custom window decorations and frontend typescript support using SWC.

