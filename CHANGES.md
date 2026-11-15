# woad.Rust - Changes <!-- omit in toc -->


## 0.1.0-alpha1 - 16th November 2026

* Added `BOLD` and `BLINKING` SGR sequences;
* Added cached `colour_policy()`, `refresh()`, `is_tty()`, `supports_colour()`, and `seq_for()` for the portable half of the **woad** 0.3.0 colour contract (Unix terminal attachment via `std::io::IsTerminal`);
* Windows virtual-terminal mode and mintty PTY recognition are not in this alpha;
* Documented the colour environment variables and their accepted forms in **README.md**, including Developer Experience commands that force or suppress colour and that pipe the example to `cat`;
* Moved the fixed SGR sequences into private **src/sequences.rs**, re-exported at the crate root;


## 0.0.1 - 12th September 2026

* SGR colour and reset codes (`RESET`, `FG_*`, `BG_*`, including bright variants);


## 0.0.0 - 15th August 2026

* initial project scaffolding;


<!-- ########################### end of file ########################### -->
