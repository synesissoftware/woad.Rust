# woad.Rust - Changes <!-- omit in toc -->


## 0.1.0-alpha2 - 19th November 2026

* Added packed `version()` and `version_string()` in the **woad** `WOAD_VER` layout (`0x00010042`; relstatus `0x42` is alpha 2), with `VERSION_MAJOR`, `VERSION_MINOR`, `VERSION_PATCH`, and `VERSION_RELSTATUS`, in **src/version.rs**;
* Added `ColourPolicy::as_str` and `Display` (`auto`, `force`, `inhibit`);
* Added attribute helpers `blinking_for`, `bold_for`, and `reset_for`;
* Added Unix descriptor forms `fd_is_tty`, `fd_supports_colour`, and `seq_for_fd`;
* Added null-stream forms `supports_colour_absent` and `seq_for_absent`;
* Added `console_supports_ansi()`, which reports `true` in this alpha;
* Added **examples/colour_for/main.rs**, covering stdout, stderr, the attribute helpers, and, on Unix, the descriptor form;
* **examples/versions/main.rs** prints `version_string()` and packed `version()` with colour;


## 0.1.0-alpha1 - 16th November 2026

* Added `BOLD` and `BLINKING` SGR sequences;
* Added cached `colour_policy()` (returning `ColourPolicy`, discriminants 0, 1, and 2), `refresh()`, `is_tty()`, `supports_colour()`, and `seq_for()` for the portable half of the **woad** 0.3.0 colour contract (Unix terminal attachment via `std::io::IsTerminal`);
* Windows virtual-terminal mode and mintty PTY recognition are not in this alpha;
* Documented the colour environment variables and their accepted forms in **README.md**, including Developer Experience commands that force or suppress colour and that pipe the example to `cat`;
* Moved the fixed SGR sequences into private **src/sequences.rs**, re-exported at the crate root;


## 0.0.1 - 12th September 2026

* SGR colour and reset codes (`RESET`, `FG_*`, `BG_*`, including bright variants);


## 0.0.0 - 15th August 2026

* initial project scaffolding;


<!-- ########################### end of file ########################### -->
