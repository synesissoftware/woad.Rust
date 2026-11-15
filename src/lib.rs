//! Minimal ANSI terminal colour codes, for Rust.
//!
//! **woad** provides the smallest useful set of fixed SGR sequences for
//! library authors, and the portable half of the colour policy: cached
//! environment inspection, `TERM=dumb`, and conditional emission for a
//! writer that implements [`std::io::IsTerminal`]. It is not a console or
//! TUI framework. Windows virtual-terminal gating is a later alpha.

mod policy;
mod sequences;

pub use policy::{
    colour_policy,
    is_tty,
    refresh,
    seq_for,
    supports_colour,
    COLOUR_POLICY_AUTO,
    COLOUR_POLICY_FORCE,
    COLOUR_POLICY_INHIBIT,
};
pub use sequences::{
    BG_BLACK,
    BG_BLUE,
    BG_BRIGHT_BLACK,
    BG_BRIGHT_BLUE,
    BG_BRIGHT_CYAN,
    BG_BRIGHT_GREEN,
    BG_BRIGHT_MAGENTA,
    BG_BRIGHT_RED,
    BG_BRIGHT_WHITE,
    BG_BRIGHT_YELLOW,
    BG_CYAN,
    BG_GREEN,
    BG_MAGENTA,
    BG_RED,
    BG_WHITE,
    BG_YELLOW,
    BLINKING,
    BOLD,
    FG_BLACK,
    FG_BLUE,
    FG_BRIGHT_BLACK,
    FG_BRIGHT_BLUE,
    FG_BRIGHT_CYAN,
    FG_BRIGHT_GREEN,
    FG_BRIGHT_MAGENTA,
    FG_BRIGHT_RED,
    FG_BRIGHT_WHITE,
    FG_BRIGHT_YELLOW,
    FG_CYAN,
    FG_GREEN,
    FG_MAGENTA,
    FG_RED,
    FG_WHITE,
    FG_YELLOW,
    RESET,
};


/// Crate version.
pub const VERSION : &str = "0.1.0-alpha1";


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]


    #[test]
    fn TEST_VERSION() {
        assert_eq!(crate::VERSION, "0.1.0-alpha1");
    }

    #[test]
    fn TEST_RESET() {
        assert_eq!(crate::RESET, "\x1b[0m");
    }

    #[test]
    fn TEST_BOLD() {
        assert_eq!(crate::BOLD, "\x1b[1m");
    }

    #[test]
    fn TEST_BLINKING() {
        assert_eq!(crate::BLINKING, "\x1b[5m");
    }

    #[test]
    fn TEST_FG_RED() {
        assert_eq!(crate::FG_RED, "\x1b[31m");
    }

    #[test]
    fn TEST_BG_BLUE() {
        assert_eq!(crate::BG_BLUE, "\x1b[44m");
    }

    #[test]
    fn TEST_CODES_ARE_CSI_SGR() {
        let codes = [
            crate::RESET,
            crate::BOLD,
            crate::BLINKING,
            crate::FG_BLACK,
            crate::FG_BRIGHT_WHITE,
            crate::BG_BLACK,
            crate::BG_BRIGHT_WHITE,
        ];

        for code in codes {
            assert!(code.starts_with("\x1b["));
            assert!(code.ends_with('m'));
        }
    }
}
