# Glove80 shortcut listener

Read README.md for the supported TOML configuration and commands. Use example.toml as a starter. The `[layers.special]` table refers to the firmware's **Custom** layer. The firmware also has a separate layer named **Special** that the editor shows as part of the cheat sheet.

- The Windows listener and local web editor run as separate processes. The listener validates TOML in src/config.rs. Reuse that validation for any new config-writing path.
- fixtures/layout.json maps the 26 physical letter slots to Windows chords. ui/keyboard.json snapshots the firmware positions and per-layer bindings in firmware order. ui/legends.js supplies display labels and thumb geometry. Keep the browser labels readable and preserve the firmware indices.
- ui/moonlander.json snapshots the 72 keys in `firmware/moonlander/keymap.c` order. It uses readable labels and the geometry from the Moonlander layout diagrams. Keep the layer bindings and the 26 listener chords in sync with the QMK keymap and fixtures/layout.json.
- Custom-layer keys can be assigned existing actions, or used to create and edit named actions. One action can serve multiple bindings. In the UI, Use this action stages edits and Save changes persists them. Preserve unrelated TOML sections and reject stale saves.
- Build with `cargo build --release --locked`. Use `glove80-listener validate <config.toml>` after changing the schema or examples. The desktop integration test is tests/Test-Listener.ps1 and sends synthetic F13 presses. For editor changes, test against a copy of the config.
