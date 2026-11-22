# Glove80 shortcut listener

A per-user Windows listener written in Rust. Bind special-layer slots to named actions in TOML. Start with example.toml and substitute your app pipe and script paths.

    cargo build --release --locked
    .\target\release\glove80-listener.exe validate example.toml
    .\target\release\glove80-listener.exe run example.toml

Keep the listener running in a terminal. Ctrl+C exits. Only configured slots reserve global hotkeys. Any keyboard producing the same chord can trigger them. The special-layer letters follow fixtures/layout.json. The config is checked once per second. A failed reload leaves the previous bindings active. Restart or edit again to retry a registration conflict.

Configure named pipes under apps. An app.command action connects to the local pipe on each press, writes one newline-terminated UTF-8 JSON object with command and args, then closes its handle. The app should accept successive connections. Delivery is one-way. A successful write confirms delivery to the pipe buffer. Errors go to stderr and the listener does not retry commands. Each pipe has a separate worker and bounded queue. process.run launches an executable with individual arguments and optional cwd. window.focus_or_launch uses process (executable basename), exe, and optional args. It targets the first visible unowned matching window in enumeration order. Windows decides whether to grant foreground focus.

To exercise the compiled listener on an interactive desktop with F13 available:

    powershell -NoProfile -ExecutionPolicy RemoteSigned -File .\tests\Test-Listener.ps1

The following earlier probes can also be run independently.

Run from PowerShell in the repository root:

    powershell -NoProfile -ExecutionPolicy RemoteSigned -File .\tests\Test-Layout.ps1
    powershell -NoProfile -ExecutionPolicy RemoteSigned -File .\tests\Test-WindowsHotkey.ps1
    powershell -NoProfile -ExecutionPolicy RemoteSigned -File .\tests\Test-WindowsHotkey.ps1 -WaitForPress

The layout fixture is proposed test data. The first test checks all 26 letter slots and rejects duplicated or malformed chords. The Windows probe briefly reserves F13 with repeat suppression. Interactive mode waits up to 20 seconds for a real press and can receive it while another application has focus. Each run releases the registration on exit. The probe never launches an action. A successful reservation only describes availability on the current desktop at test time.

Run the end-to-end listener test without a physical shortcut using `powershell -NoProfile -ExecutionPolicy RemoteSigned -File .\tests\Test-WindowsHotkey.ps1 -Emulate`. It registers F13, injects a key-down and key-up with Windows SendInput, and verifies delivery of WM_HOTKEY.
The custom-command demo starts a mock app, connects a local named pipe, emulates F13, sends the JSON command from fixtures/custom-command.json, and checks the resulting counter. Run `powershell -NoProfile -ExecutionPolicy RemoteSigned -File .\tests\Test-CustomCommand.ps1`. The wire format is one JSON object per line in each direction.


## Layout editor

Launch the local editor with the TOML file you want to edit:

    .\target\release\glove80-editor.exe example.toml

Open http://127.0.0.1:8765. The cheat sheet shows Base, Special, Movement, Magic, and Custom from firmware commit 2fce90f. The two highlighted thumb keys activate Custom. On that layer, select a letter and assign any named action from your TOML file. The action editor opens for the selected Custom key. The Windows action dropdown lists saved actions plus Edit selected action and Create new action. Use this action stages the change. Save changes writes the TOML file. An edit to a shared action affects every key assigned to it. Reload reads any edits made directly in TOML. Ctrl+C in the editor terminal stops the server.

### Configuration format

The listener reads TOML version 1. The three sections below define the keys, the actions, and any named-pipe endpoints. Multiple keys may reference the same action. Unassigned keys have no entry. Run the listener with `validate <config.toml>` to check a config.

```toml
version = 1

[layers.special]
A = "start_timer"
D = "start_timer"
S = "run_script"

[apps.timer]
pipe = "my-timer.commands"

[actions.start_timer]
label = "Start focus timer"
type = "app.command"
app = "timer"
command = "start_timer"
args = { minutes = 25 }

[actions.run_script]
label = "Open workspace"
type = "process.run"
exe = "powershell.exe"
args = ["-NoProfile", "-NonInteractive", "-File", 'C:\Scripts\Open-Workspace.ps1']

[actions.open_editor]
label = "Bring editor forward"
type = "window.focus_or_launch"
process = "Code.exe"
exe = "Code.exe"
args = []
```

- `app.command` accepts an optional argument table. The target app opens the named pipe, accepts successive connections, and handles a newline-terminated UTF-8 JSON message such as `{"command":"start_timer","args":{"minutes":25}}`.
- `process.run` accepts an optional array of string arguments and an optional working directory in `cwd`. Use a full executable path when it is unavailable on PATH.
- `window.focus_or_launch` matches the process by executable basename and uses `exe` and optional string arguments if it needs to launch. Windows controls foreground activation.

## Keyboard layouts

- [Glove80 layer diagrams](docs/layouts/glove80.md) show Base, Special, Movement, Magic, and Custom.
- [Moonlander layer diagrams](docs/layouts/moonlander.md) show the four layers in the current QMK source.

## Moonlander firmware

See [Moonlander firmware](firmware/moonlander/README.md) for the QMK port of the Glove80 Custom shortcut layer.
