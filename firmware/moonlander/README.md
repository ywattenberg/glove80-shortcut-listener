# Moonlander firmware

This QMK keymap starts from the compiled Oryx `dev` layout, revision `nlWmgK`. Layer 3 is the Glove80 Custom shortcut layer. The existing Moonlander thumb keys that activate layer 3 use one-shot switching. Tap either switch and then the corresponding QWERTY letter to send the chord in `fixtures/layout.json`. The Windows listener uses the same configured actions for both keyboards.

Layers 0–2 and the RGB settings currently come from the Moonlander baseline. The full Glove80 port still needs a decision on the Glove80 thumb and outer-row functions that have no matching dedicated key on the Moonlander.

Use [ZSA's QMK fork](https://github.com/zsa/qmk_firmware) to build the keymap. Copy the three source files to `keyboards/zsa/moonlander/keymaps/glove80_port/` in that checkout. Run `qmk list-keyboards` to identify the Moonlander target for your hardware revision, then compile with `qmk compile -kb <target> -km glove80_port`. This keymap is maintained as QMK source. The Oryx revision remains available as the firmware rollback.
