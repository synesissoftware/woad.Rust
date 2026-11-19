# woad.Rust - Changes <!-- omit in toc -->


## 0.1.0-alpha1 - 16th November 2026

* Added `BOLD` and `BLINKING` SGR sequences, kept in private **src/sequences.rs** and re-exported at the crate root;
* Cached `colour_policy()` (returning `ColourPolicy`, discriminants 0, 1, and 2) from `WOAD_COLOUR` / `WOAD_COLOR`, `NO_COLOR`, `CLICOLOR_FORCE`, `FORCE_COLOR`, and `CLICOLOR`, held until `refresh()`;
* Stream-conditional sequences (`seq_for`, `supports_colour`, `is_tty`, attribute helpers) for `IsTerminal` destinations, plus Unix `AsFd` equivalents of the descriptor API;
* Packed `version()` in the **woad** `WOAD_VER` layout (`0x41` alpha 1) and `version_string()`;
* Documented the colour environment variables and their accepted forms in **README.md**, including Developer Experience commands that force or suppress colour and that pipe the example to `cat`;
* Windows virtual-terminal mode, console-mode changes, handle currencies, and mintty PTY recognition are not in this alpha;


## 0.0.1 - 12th September 2026

* SGR colour and reset codes (`RESET`, `FG_*`, `BG_*`, including bright variants);


## 0.0.0 - 15th August 2026

* initial project scaffolding;


<!-- ########################### end of file ########################### -->
