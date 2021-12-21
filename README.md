# Skyrim `controlmap.txt` Editor
Skyrim has a custom file format for configuring controls.

There aren't many tools or guides for making custom gamepad controls. A lot of players use [Gamepad++](https://www.nexusmods.com/skyrimspecialedition/mods/27007), but it doesn't enable rebinding of core controls.

The best tool the Skyrim modding community has had for custom gamepad controls is [Xbox 360 Controller Remapper](https://www.nexusmods.com/skyrim/mods/35405). It's a bit crusty, old, and has some rough edges.

There historically hasn't been a way to merge `controlmap.txt` files, meaning that mods that want to ship custom controls can't do so without overwriting all of them.

This is a command line tool that:
- Edits `controlmap.txt` files from the command line
- Merges `controlmap.txt` files together to allow patching
- Reformats `controlmap.txt` files to make them easier to read
- Interprets `controlmap.txt` files and shows human-readable descriptions of bindings

## License
This project is available under the [Mozilla Public License, Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/). Details are available in [`LICENSE.txt`](LICENSE.txt).