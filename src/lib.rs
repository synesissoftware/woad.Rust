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

pub use dynamic::{
    bg_for,
    blinking_for,
    bold_for,
    console_supports_ansi,
    fg_for,
    reset_for,
    seq_for_absent,
    seq_for_stream,
    stream_is_tty,
    stream_supports_colour,
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


/// Major component of [`version`].
pub const VERSION_MAJOR : u32 = 0;

/// Minor component of [`version`].
pub const VERSION_MINOR : u32 = 1;

/// Patch component of [`version`].
pub const VERSION_PATCH : u32 = 0;

/// Release-status byte of [`version`].
///
/// `0x41` is alpha 1. `0xFF` is a numbered release. The layout matches
/// **woad** `WOAD_VER`: bits 24-31 major, 16-23 minor, 8-15 patch, and
/// 0-7 release status.
pub const VERSION_RELSTATUS : u32 = 0x41;

/// Crate version, SemVer, including the prerelease label.
pub const VERSION : &str = "0.1.0-alpha1";

/// Packed **woad** version, in the `WOAD_VER` layout.
///
/// ```
/// assert_eq!(0x0001_0041, woad::version());
/// assert_eq!(woad::VERSION, woad::version_string());
/// ```
pub const fn version() -> u32 {
    (VERSION_MAJOR << 24) | (VERSION_MINOR << 16) | (VERSION_PATCH << 8) | VERSION_RELSTATUS
}

/// Version string, including the prerelease label.
///
/// This is [`VERSION`]. The C `WOAD_VER_STRING` omits the prerelease
/// label; the low byte of [`version`] still carries it.
pub const fn version_string() -> &'static str {
    VERSION
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]


    #[test]
    fn TEST_VERSION() {
        assert_eq!("0.1.0-alpha1", crate::VERSION);
        assert_eq!(crate::VERSION, crate::version_string());
        assert_eq!(crate::VERSION, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn TEST_VERSION_PACKED() {
        assert_eq!(0, crate::VERSION_MAJOR);
        assert_eq!(1, crate::VERSION_MINOR);
        assert_eq!(0, crate::VERSION_PATCH);
        assert_eq!(0x41, crate::VERSION_RELSTATUS);
        assert_eq!(0x0001_0041, crate::version());
    }

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
