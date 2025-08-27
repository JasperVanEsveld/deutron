# Deutron Command

The deutron command is used to compile or debug your webview applications.


## Why?

Web based desktop apps have existed for a while now, why use deutron?
Deutron uses WebView, a technology available on many platforms that allows applications to open a window that loads a webpage utilizing a pre-installed broswer. Allowing for small applications with the UI power of a browser.

Implementing WebView to work with your existing programs can be cumbersome. Projects like Tauri have solved many of the technical issues but force your projects into a specific structure.
Deutron has no config files, no Rust projects or specific file structure.

## How?

Deutron launches your command and uses it's stdin/stdout to launch and control webview windows.
Using the amazing crates from Tauri, namely tao (crossplatform window manager) and wry (cross platform webview library).

This moves all the heavy lifting away from your backend. Communication between your backend and deutron is done through json messages preceded by `DEUTRON_IPC:`, to see these messages use the `--verbose` flag.
This allows you to still use print statements, as long as they don't start with `DEUTRON_IPC:`.


```
Usage: deutron [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  The command used to run your backend

Options:
  -s, --set-version <SET_VERSION>  Set the version of the output binary [default: 1.0]
      --dev-tools                  Enables the use of webview developer console (F12 on windows)
  -t, --terminal                   Opens a terminal on windows when the application is double clicked, useful for debugging
  -v, --verbose                    Logs all info and messages send
  -d, --debug                      Directly runs command instead of compiling to an executable
  -o, --out <OUT>                  The output file of the executable
  -i, --include <INCLUDE>          The directory that is packed into the binary [default: ./]
  -h, --help                       Print help
  -V, --version                    Print version
```