//! Minimal ANSI terminal colour codes, for Rust.
//!
//! **woad** provides fixed SGR sequences, and stream-conditional sequences
//! that emit only when the destination warrants colour. The decision is the
//! portable half of the **woad** 0.3.0 colour contract: cached environment
//! inspection, `TERM=dumb`, and [`std::io::IsTerminal`]. It is not a console
//! or TUI framework.
//!
//! Windows console-mode changes, virtual-terminal gating, and handle
//! currencies are not part of this alpha. [`console_supports_ansi`] is
//! always `true`, which is the non-Windows answer. On Windows a terminal is
//! treated with the Unix rule.
//!
//! ```
//! use woad::{reset_for, seq_for, FG_GREEN};
//!
//! let stdout = std::io::stdout();
//!
//! println!("{}ok{}", seq_for(FG_GREEN, &stdout), reset_for(&stdout),);
//! ```

mod dynamic;
mod policy;
mod sequences;

#[path = "version.rs"]
mod crate_version;
pub mod version;

pub use dynamic::{
    blinking_for,
    bold_for,
    console_supports_ansi,
    reset_for,
    seq_for_absent,
    supports_colour_absent,
};
pub use policy::{
    colour_policy,
    is_tty,
    refresh,
    seq_for,
    supports_colour,
    ColourPolicy,
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

#[cfg(unix)]
pub use dynamic::{
    fd_is_tty,
    fd_supports_colour,
    seq_for_fd,
};
pub use crate_version::{
    VERSION,
    VERSION_MAJOR,
    VERSION_MINOR,
    VERSION_PATCH,
    VERSION_RELSTATUS,
    version,
    version_string,
};


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]


    #[test]
    fn TEST_RESET() {
        assert_eq!("\x1b[0m", crate::RESET);
    }

    #[test]
    fn TEST_BOLD() {
        assert_eq!("\x1b[1m", crate::BOLD);
    }

    #[test]
    fn TEST_BLINKING() {
        assert_eq!("\x1b[5m", crate::BLINKING);
    }

    #[test]
    fn TEST_FG_RED() {
        assert_eq!("\x1b[31m", crate::FG_RED);
    }

    #[test]
    fn TEST_BG_BLUE() {
        assert_eq!("\x1b[44m", crate::BG_BLUE);
    }

    #[test]
    fn TEST_CODES_ARE_CSI_SGR() {
        let codes = [
            crate::BLINKING,
            crate::BOLD,
            crate::RESET,
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
