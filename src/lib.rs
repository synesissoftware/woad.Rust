//! Minimal ANSI terminal colour codes, for Rust.
//!
//! **woad** provides fixed SGR sequences, and stream-conditional
//! sequences that emit only when the destination warrants colour. The
//! decision follows the same environment policy as **woad** for C.
//! It is not a console or TUI framework.
//!
//! Windows console-mode changes and handle currencies are not part of
//! this alpha. [`console_supports_ansi`] is always `true`, which is the
//! non-Windows answer.
//!
//! ```
//! use woad::{reset_for, seq_for, FG_GREEN};
//!
//! let stdout = std::io::stdout();
//!
//! println!("{}ok{}", seq_for(FG_GREEN, &stdout), reset_for(&stdout),);
//! ```

mod dynamic;

pub use dynamic::{
    bg_for,
    blinking_for,
    bold_for,
    colour_policy,
    console_supports_ansi,
    fg_for,
    refresh,
    reset_for,
    seq_for,
    seq_for_absent,
    seq_for_stream,
    stream_is_tty,
    stream_supports_colour,
    supports_colour,
    supports_colour_absent,
    ColourPolicy,
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


// Reset

/// Reset all attributes.
pub const RESET : &str = "\x1b[0m";

/// Bold / increased intensity.
pub const BOLD : &str = "\x1b[1m";

/// Slow blink.
pub const BLINKING : &str = "\x1b[5m";


// Foreground (standard)

/// Foreground black.
pub const FG_BLACK : &str = "\x1b[30m";
/// Foreground red.
pub const FG_RED : &str = "\x1b[31m";
/// Foreground green.
pub const FG_GREEN : &str = "\x1b[32m";
/// Foreground yellow.
pub const FG_YELLOW : &str = "\x1b[33m";
/// Foreground blue.
pub const FG_BLUE : &str = "\x1b[34m";
/// Foreground magenta.
pub const FG_MAGENTA : &str = "\x1b[35m";
/// Foreground cyan.
pub const FG_CYAN : &str = "\x1b[36m";
/// Foreground white.
pub const FG_WHITE : &str = "\x1b[37m";


// Foreground (bright)

/// Foreground bright black.
pub const FG_BRIGHT_BLACK : &str = "\x1b[90m";
/// Foreground bright red.
pub const FG_BRIGHT_RED : &str = "\x1b[91m";
/// Foreground bright green.
pub const FG_BRIGHT_GREEN : &str = "\x1b[92m";
/// Foreground bright yellow.
pub const FG_BRIGHT_YELLOW : &str = "\x1b[93m";
/// Foreground bright blue.
pub const FG_BRIGHT_BLUE : &str = "\x1b[94m";
/// Foreground bright magenta.
pub const FG_BRIGHT_MAGENTA : &str = "\x1b[95m";
/// Foreground bright cyan.
pub const FG_BRIGHT_CYAN : &str = "\x1b[96m";
/// Foreground bright white.
pub const FG_BRIGHT_WHITE : &str = "\x1b[97m";


// Background (standard)

/// Background black.
pub const BG_BLACK : &str = "\x1b[40m";
/// Background red.
pub const BG_RED : &str = "\x1b[41m";
/// Background green.
pub const BG_GREEN : &str = "\x1b[42m";
/// Background yellow.
pub const BG_YELLOW : &str = "\x1b[43m";
/// Background blue.
pub const BG_BLUE : &str = "\x1b[44m";
/// Background magenta.
pub const BG_MAGENTA : &str = "\x1b[45m";
/// Background cyan.
pub const BG_CYAN : &str = "\x1b[46m";
/// Background white.
pub const BG_WHITE : &str = "\x1b[47m";


// Background (bright)

/// Background bright black.
pub const BG_BRIGHT_BLACK : &str = "\x1b[100m";
/// Background bright red.
pub const BG_BRIGHT_RED : &str = "\x1b[101m";
/// Background bright green.
pub const BG_BRIGHT_GREEN : &str = "\x1b[102m";
/// Background bright yellow.
pub const BG_BRIGHT_YELLOW : &str = "\x1b[103m";
/// Background bright blue.
pub const BG_BRIGHT_BLUE : &str = "\x1b[104m";
/// Background bright magenta.
pub const BG_BRIGHT_MAGENTA : &str = "\x1b[105m";
/// Background bright cyan.
pub const BG_BRIGHT_CYAN : &str = "\x1b[106m";
/// Background bright white.
pub const BG_BRIGHT_WHITE : &str = "\x1b[107m";


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
