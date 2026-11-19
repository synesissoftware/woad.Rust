//! Extra destination currencies from the earlier alpha.
//!
//! Policy and `seq_for` live in [`crate::policy`]. These names are the
//! stream, descriptor, and absent-destination forms, plus the attribute
//! helpers. Windows handle currencies are not implemented.

#[cfg(unix)]
use std::os::fd::AsFd;
use std::io::IsTerminal;

use crate::{
    policy,
    seq_for,
    supports_colour,
    BLINKING,
    BOLD,
    RESET,
};


/// Whether ANSI SGR sequences are known to be interpreted.
///
/// Off Windows, **woad** always reports that they are. Virtual-terminal
/// detection and `WOAD_SET_CONSOLE_MODE` are not implemented in this alpha,
/// so this function does not consult the environment.
pub fn console_supports_ansi() -> bool {
    true
}

/// Whether `target` is attached to a terminal.
///
/// This is [`std::io::IsTerminal`] and does not apply the colour policy.
pub fn stream_is_tty(target : &impl IsTerminal) -> bool {
    target.is_terminal()
}

/// Whether `fd` is attached to a terminal.
///
/// Unix file-descriptor currency. Windows handle currencies are not
/// implemented.
#[cfg(unix)]
pub fn fd_is_tty(fd : impl AsFd) -> bool {
    stream_is_tty(&fd.as_fd())
}

/// Whether colour should be emitted for `target`.
pub fn stream_supports_colour(target : &impl IsTerminal) -> bool {
    supports_colour(target)
}

/// Whether colour should be emitted for `fd`.
#[cfg(unix)]
pub fn fd_supports_colour(fd : impl AsFd) -> bool {
    supports_colour(&fd.as_fd())
}

/// Whether colour should be emitted when no destination is available.
///
/// This is the null-stream case: the policy alone, with no terminal
/// attachment. Force is the only policy that yields `true`.
pub fn supports_colour_absent() -> bool {
    policy::supports_when_attached(false)
}

/// Yields `seq` when `target` warrants colour, and `""` otherwise.
pub fn seq_for_stream<'seq>(
    seq : &'seq str,
    target : &impl IsTerminal,
) -> &'seq str {
    seq_for(seq, target)
}

/// Descriptor equivalent of [`seq_for_stream`].
#[cfg(unix)]
pub fn seq_for_fd(
    seq : &str,
    fd : impl AsFd,
) -> &str {
    seq_for(seq, &fd.as_fd())
}

/// Yields `seq` from the policy alone, as for a null stream.
pub fn seq_for_absent(seq : &str) -> &str {
    if seq.is_empty() || !supports_colour_absent() {
        ""
    } else {
        seq
    }
}

/// Foreground-intent synonym of [`seq_for`].
pub fn fg_for<'seq>(
    seq : &'seq str,
    target : &impl IsTerminal,
) -> &'seq str {
    seq_for(seq, target)
}

/// Background-intent synonym of [`seq_for`].
pub fn bg_for<'seq>(
    seq : &'seq str,
    target : &impl IsTerminal,
) -> &'seq str {
    seq_for(seq, target)
}

/// Yields [`RESET`] for `target`, or `""`.
pub fn reset_for(target : &impl IsTerminal) -> &'static str {
    seq_for(RESET, target)
}

/// Yields [`BOLD`] for `target`, or `""`.
pub fn bold_for(target : &impl IsTerminal) -> &'static str {
    seq_for(BOLD, target)
}

/// Yields [`BLINKING`] for `target`, or `""`.
pub fn blinking_for(target : &impl IsTerminal) -> &'static str {
    seq_for(BLINKING, target)
}
