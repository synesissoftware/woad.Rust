# woad.Rust - Changes <!-- omit in toc -->


## 0.1.0-alpha1 - 22nd October 2026

* Colour policy from `WOAD_COLOUR` / `WOAD_COLOR`, `NO_COLOR`, `CLICOLOR_FORCE`, and `CLICOLOR`, cached until `refresh()`;
* Stream-conditional sequences (`seq_for`, `supports_colour`, attribute helpers) for `IsTerminal` destinations, plus Unix `AsFd` equivalents of the descriptor API;
* Packed `version()` in the **woad** `WOAD_VER` layout (`0x41` alpha 1) and `version_string()`;
* Attribute sequences `BOLD` and `BLINKING`;
* Windows console-mode and handle currencies are left for a later alpha;


## 0.0.1 - 12th September 2026

* SGR colour and reset codes (`RESET`, `FG_*`, `BG_*`, including bright variants);


## 0.0.0 - 15th August 2026

* initial project scaffolding;


<!-- ########################### end of file ########################### -->
