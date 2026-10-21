//! Stream-conditional colour: policy, terminal attachment, and sequence
//! selection.
//!
//! This is the dynamic half of **woad**. Fixed sequences live in the
//! crate root. Windows console handles and virtual-terminal opt-in are
//! not implemented; see [`console_supports_ansi`].

use std::{
    io::IsTerminal,
    sync::Mutex,
};

#[cfg(unix)]
use std::os::fd::AsFd;


// The truey/falsey token sets come from 2be
// (https://github.com/synesissoftware/2be). Classification is local so
// this crate stays dependency-free. woad adds `always` / `force` and
// `never` / `none`, compares case-insensitively, and does not trim.


const TRUEY : &[&str] = &["1", "always", "force", "on", "true", "yes"];

const FALSEY : &[&str] = &["0", "false", "never", "no", "none", "off"];


/// Prevailing colour policy, matching `WOAD_COLOUR_POLICY_*`.
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[repr(i32)]
pub enum ColourPolicy {
    /// Defer to the destination: a terminal that is not `TERM=dumb`.
    Auto = 0,
    /// Emit colour for every destination.
    Force = 1,
    /// Emit colour for no destination.
    Inhibit = 2,
}

impl ColourPolicy {
    /// Stable policy name: `auto`, `force`, or `inhibit`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Force => "force",
            Self::Inhibit => "inhibit",
        }
    }
}

impl std::fmt::Display for ColourPolicy {
    fn fmt(
        &self,
        f : &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}


struct Cached {
    policy :       ColourPolicy,
    term_is_dumb : bool,
}

static CACHE : Mutex<Option<Cached>> = Mutex::new(None);

#[cfg(test)]
mod test_env {
    use std::{
        cell::RefCell,
        collections::BTreeMap,
    };

    // `const { }` in `thread_local!` needs Rust 1.79; the crate MSRV is
    // 1.74, and this overlay exists only for tests.
    thread_local! {
        #[allow(clippy::missing_const_for_thread_local)]
        pub(super) static ENV_OVERLAY : RefCell<Option<BTreeMap<String, String>>> =
            RefCell::new(None);
    }

    pub(super) fn overlay_active() -> bool {
        ENV_OVERLAY.with(|slot| slot.borrow().is_some())
    }
}


fn lock_cache() -> std::sync::MutexGuard<'static, Option<Cached>> {
    CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn getenv_nonempty(name : &str) -> Option<String> {
    #[cfg(test)]
    {
        let overlaid = test_env::ENV_OVERLAY.with(|slot| {
            slot.borrow()
                .as_ref()
                .map(|map| map.get(name).filter(|value| !value.is_empty()).cloned())
        });

        if let Some(value) = overlaid {
            return value;
        }
    }

    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn is_truey(value : &str) -> bool {
    TRUEY.iter().any(|token| value.eq_ignore_ascii_case(token))
}

fn is_falsey(value : &str) -> bool {
    FALSEY.iter().any(|token| value.eq_ignore_ascii_case(token))
}

fn policy_from_environment() -> ColourPolicy {
    let value = getenv_nonempty("WOAD_COLOUR").or_else(|| getenv_nonempty("WOAD_COLOR"));

    if let Some(value) = value.as_deref() {
        if is_truey(value) {
            return ColourPolicy::Force;
        }

        if is_falsey(value) {
            return ColourPolicy::Inhibit;
        }

        return ColourPolicy::Auto;
    }

    if getenv_nonempty("NO_COLOR").is_some() {
        return ColourPolicy::Inhibit;
    }

    if let Some(value) = getenv_nonempty("CLICOLOR_FORCE") {
        if value != "0" {
            return ColourPolicy::Force;
        }
    }

    if let Some(value) = getenv_nonempty("CLICOLOR") {
        if value == "0" {
            return ColourPolicy::Inhibit;
        }
    }

    ColourPolicy::Auto
}

fn term_is_dumb_from_environment() -> bool {
    getenv_nonempty("TERM").is_some_and(|value| value.eq_ignore_ascii_case("dumb"))
}

fn current_determination() -> (ColourPolicy, bool) {
    #[cfg(test)]
    if test_env::overlay_active() {
        return (policy_from_environment(), term_is_dumb_from_environment());
    }

    let mut cache = lock_cache();

    if let Some(cached) = cache.as_ref() {
        return (cached.policy, cached.term_is_dumb);
    }

    let policy = policy_from_environment();
    let term_is_dumb = term_is_dumb_from_environment();

    *cache = Some(Cached {
        policy,
        term_is_dumb,
    });

    (policy, term_is_dumb)
}

fn colour_is_warranted(
    is_tty : bool,
    policy : ColourPolicy,
    term_is_dumb : bool,
    console_ansi : bool,
) -> bool {
    match policy {
        ColourPolicy::Force => true,
        ColourPolicy::Inhibit => false,
        ColourPolicy::Auto => is_tty && !term_is_dumb && console_ansi,
    }
}

fn colour_for_attachment(is_tty : bool) -> bool {
    let (policy, term_is_dumb) = current_determination();

    colour_is_warranted(is_tty, policy, term_is_dumb, console_supports_ansi())
}

fn seq_when(
    seq : &str,
    warranted : bool,
) -> &str {
    if warranted {
        seq
    } else {
        ""
    }
}


/// Obtains the prevailing colour policy, from the environment.
///
/// The environment is read once, on first need, and cached until
/// [`refresh`]. Later queries in the process see that cached policy.
/// Variables are consulted in this order, and the first decisive one
/// wins:
///
/// * `WOAD_COLOUR`, or, if that is unset or empty, `WOAD_COLOR`. A
///   truey value (`1`, `always`, `force`, `on`, `true`, `yes`) forces
///   colour. A falsey value (`0`, `false`, `never`, `no`, `none`,
///   `off`) inhibits it. `auto`, or any unrecognised value, selects
///   [`ColourPolicy::Auto`]. Comparison is case-insensitive;
/// * `NO_COLOR`, when set and non-empty, inhibits colour
///   (<https://no-color.org/>);
/// * `CLICOLOR_FORCE`, when set, non-empty, and not exactly `0`, forces
///   colour. This comparison is case-sensitive;
/// * `CLICOLOR`, when set to exactly `0`, inhibits colour;
///
/// Under [`ColourPolicy::Auto`], a `TERM` of `dumb`
/// (case-insensitive) also suppresses colour for a terminal.
///
/// An empty value is treated as unset. Values are not trimmed.
pub fn colour_policy() -> ColourPolicy {
    current_determination().0
}

/// Discards the cached policy and `TERM` inspection.
///
/// The next query reads the environment again. Queries are
/// synchronised with this call. Mutating the process environment
/// concurrently with a query is the caller's concern, as it is for
/// any other reader of the environment.
pub fn refresh() {
    *lock_cache() = None;
}

/// Whether ANSI SGR sequences are known to be interpreted.
///
/// Off Windows, **woad** always reports that they are.
/// Virtual-terminal detection, and `WOAD_SET_CONSOLE_MODE`, are
/// Windows-only and are not implemented in this alpha, so this
/// function does not consult the environment.
pub fn console_supports_ansi() -> bool {
    true
}

/// Whether `target` is attached to a terminal.
///
/// This is [`std::io::IsTerminal`] for the Rust stream currency
/// (`Stdout`, `Stderr`, `Stdin`, [`std::fs::File`], and, on Unix, a
/// borrowed file descriptor).
pub fn stream_is_tty(target : &impl IsTerminal) -> bool {
    target.is_terminal()
}

/// Whether `fd` is attached to a terminal.
///
/// Unix file-descriptor currency, equivalent to `woad_fd_is_tty`.
/// Pass a [`std::os::fd::AsFd`] value, for example `&file` or
/// `stdout.as_fd()`. Windows handle currencies are not implemented.
#[cfg(unix)]
pub fn fd_is_tty(fd : impl AsFd) -> bool {
    stream_is_tty(&fd.as_fd())
}

/// Whether colour should be emitted for `target`.
///
/// Combines [`colour_policy`], terminal attachment, `TERM=dumb`, and
/// [`console_supports_ansi`].
pub fn stream_supports_colour(target : &impl IsTerminal) -> bool {
    colour_for_attachment(stream_is_tty(target))
}

/// Whether colour should be emitted for `fd`.
///
/// Descriptor equivalent of [`stream_supports_colour`].
#[cfg(unix)]
pub fn fd_supports_colour(fd : impl AsFd) -> bool {
    stream_supports_colour(&fd.as_fd())
}

/// Whether colour should be emitted when no destination is available.
///
/// This is `woad_stream_supports_colour(NULL)`: the policy alone, with
/// no terminal attachment. [`ColourPolicy::Force`] is the only policy
/// that yields `true`.
pub fn supports_colour_absent() -> bool {
    colour_for_attachment(false)
}

/// Type-dispatched form of [`stream_supports_colour`].
///
/// On Unix a borrowed file descriptor implements [`IsTerminal`], so
/// this accepts that currency as well as standard streams and files.
pub fn supports_colour(target : &impl IsTerminal) -> bool {
    stream_supports_colour(target)
}

/// Yields `seq` when `target` warrants colour, and `""` otherwise.
///
/// The result is either `seq` or an empty string literal. An empty
/// `seq` stays empty. There is no null sequence in Rust.
pub fn seq_for_stream<'seq>(
    seq : &'seq str,
    target : &impl IsTerminal,
) -> &'seq str {
    seq_when(seq, stream_supports_colour(target))
}

/// Descriptor equivalent of [`seq_for_stream`].
#[cfg(unix)]
pub fn seq_for_fd(
    seq : &str,
    fd : impl AsFd,
) -> &str {
    seq_for_stream(seq, &fd.as_fd())
}

/// Yields `seq` from the policy alone, as for a null stream.
///
/// See [`supports_colour_absent`].
pub fn seq_for_absent(seq : &str) -> &str {
    seq_when(seq, supports_colour_absent())
}

/// Type-dispatched form of [`seq_for_stream`].
///
/// [`fg_for`] and [`bg_for`] are synonyms: the sequence already names
/// its own SGR parameter, and the split records intent at the call.
pub fn seq_for<'seq>(
    seq : &'seq str,
    target : &impl IsTerminal,
) -> &'seq str {
    seq_for_stream(seq, target)
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

/// Yields [`crate::RESET`] for `target`, or `""`.
pub fn reset_for(target : &impl IsTerminal) -> &'static str {
    seq_for(crate::RESET, target)
}

/// Yields [`crate::BOLD`] for `target`, or `""`.
pub fn bold_for(target : &impl IsTerminal) -> &'static str {
    seq_for(crate::BOLD, target)
}

/// Yields [`crate::BLINKING`] for `target`, or `""`.
pub fn blinking_for(target : &impl IsTerminal) -> &'static str {
    seq_for(crate::BLINKING, target)
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use std::io::IsTerminal;

    use super::{
        colour_is_warranted,
        colour_policy,
        console_supports_ansi,
        refresh,
        seq_for,
        seq_for_absent,
        seq_for_stream,
        stream_is_tty,
        stream_supports_colour,
        supports_colour,
        supports_colour_absent,
        test_env::ENV_OVERLAY,
        ColourPolicy,
    };
    use crate::{
        BLINKING,
        BOLD,
        FG_RED,
        RESET,
    };

    struct ClearOverlay;

    impl Drop for ClearOverlay {
        fn drop(&mut self) {
            ENV_OVERLAY.with(|slot| {
                *slot.borrow_mut() = None;
            });
        }
    }

    fn with_env<R>(
        pairs : &[(&str, &str)],
        body : impl FnOnce() -> R,
    ) -> R {
        let map = pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect();

        ENV_OVERLAY.with(|slot| {
            *slot.borrow_mut() = Some(map);
        });

        let _clear = ClearOverlay;

        body()
    }

    struct TempNotATty {
        file : Option<std::fs::File>,
        path : std::path::PathBuf,
    }

    impl TempNotATty {
        fn create() -> Self {
            use std::sync::atomic::{
                AtomicU64,
                Ordering,
            };

            static N : AtomicU64 = AtomicU64::new(0);

            let id = N.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("woad-rust-{}-{}", std::process::id(), id,));
            let file = std::fs::File::create(&path).unwrap();

            Self {
                file : Some(file),
                path,
            }
        }

        fn file(&self) -> &std::fs::File {
            self.file.as_ref().unwrap()
        }
    }

    impl Drop for TempNotATty {
        fn drop(&mut self) {
            self.file.take();
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn TEST_POLICY_EMPTY_ENVIRONMENT_IS_AUTO() {
        with_env(&[], || {
            assert_eq!(ColourPolicy::Auto, colour_policy());
            assert!(!super::term_is_dumb_from_environment());
        });
    }

    #[test]
    fn TEST_POLICY_TRUEY_TOKENS_FORCE() {
        for token in ["1", "always", "force", "on", "true", "yes", "ALWAYS", "Force"] {
            with_env(&[("WOAD_COLOUR", token)], || {
                assert_eq!(ColourPolicy::Force, colour_policy(), "{token}");
            });
        }
    }

    #[test]
    fn TEST_POLICY_FALSEY_TOKENS_INHIBIT() {
        for token in ["0", "false", "never", "no", "none", "off", "NEVER", "Off"] {
            with_env(&[("WOAD_COLOUR", token)], || {
                assert_eq!(ColourPolicy::Inhibit, colour_policy(), "{token}");
            });
        }
    }

    #[test]
    fn TEST_POLICY_UNRECOGNISED_WOAD_COLOUR_IS_AUTO() {
        for token in ["auto", "banana", " true", "2"] {
            with_env(&[("WOAD_COLOUR", token), ("NO_COLOR", "1")], || {
                assert_eq!(ColourPolicy::Auto, colour_policy(), "{token}");
            });
        }
    }

    #[test]
    fn TEST_POLICY_WOAD_COLOR_IS_ACCEPTED_WHEN_COLOUR_IS_ABSENT() {
        with_env(&[("WOAD_COLOR", "1")], || {
            assert_eq!(ColourPolicy::Force, colour_policy());
        });

        with_env(&[("WOAD_COLOUR", ""), ("WOAD_COLOR", "never")], || {
            assert_eq!(ColourPolicy::Inhibit, colour_policy());
        });

        with_env(&[("WOAD_COLOUR", "never"), ("WOAD_COLOR", "always")], || {
            assert_eq!(ColourPolicy::Inhibit, colour_policy());
        });
    }

    #[test]
    fn TEST_POLICY_PRECEDENCE() {
        with_env(&[("CLICOLOR", "0")], || {
            assert_eq!(ColourPolicy::Inhibit, colour_policy());
        });

        with_env(&[("CLICOLOR", "0"), ("CLICOLOR_FORCE", "1")], || {
            assert_eq!(ColourPolicy::Force, colour_policy());
        });

        with_env(&[("CLICOLOR_FORCE", "1"), ("NO_COLOR", "1")], || {
            assert_eq!(ColourPolicy::Inhibit, colour_policy());
        });

        with_env(
            &[("CLICOLOR_FORCE", "1"), ("NO_COLOR", "1"), ("WOAD_COLOUR", "force")],
            || {
                assert_eq!(ColourPolicy::Force, colour_policy());
            },
        );

        with_env(&[("NO_COLOR", "")], || {
            assert_eq!(ColourPolicy::Auto, colour_policy());
        });

        with_env(&[("CLICOLOR_FORCE", "0")], || {
            assert_eq!(ColourPolicy::Auto, colour_policy());
        });

        with_env(&[("CLICOLOR_FORCE", "00")], || {
            assert_eq!(ColourPolicy::Force, colour_policy());
        });

        with_env(&[("CLICOLOR", "00")], || {
            assert_eq!(ColourPolicy::Auto, colour_policy());
        });

        with_env(&[("CLICOLOR", "0 ")], || {
            assert_eq!(ColourPolicy::Auto, colour_policy());
        });
    }

    #[test]
    fn TEST_TERM_DUMB_IS_CASE_INSENSITIVE() {
        with_env(&[("TERM", "dumb")], || {
            assert!(super::term_is_dumb_from_environment());
        });

        with_env(&[("TERM", "Dumb")], || {
            assert!(super::term_is_dumb_from_environment());
        });

        with_env(&[("TERM", "xterm-256color")], || {
            assert!(!super::term_is_dumb_from_environment());
        });

        with_env(&[("TERM", "")], || {
            assert!(!super::term_is_dumb_from_environment());
        });
    }

    #[test]
    fn TEST_COLOUR_IS_WARRANTED_MATRIX() {
        let rows = [
            (true, ColourPolicy::Force, true, false, true),
            (false, ColourPolicy::Force, true, false, true),
            (true, ColourPolicy::Inhibit, false, true, false),
            (false, ColourPolicy::Inhibit, false, true, false),
            (false, ColourPolicy::Auto, false, true, false),
            (true, ColourPolicy::Auto, false, true, true),
            (true, ColourPolicy::Auto, true, true, false),
            (true, ColourPolicy::Auto, false, false, false),
            (true, ColourPolicy::Auto, true, false, false),
        ];

        for (is_tty, policy, dumb, ansi, expected) in rows {
            assert_eq!(
                expected,
                colour_is_warranted(is_tty, policy, dumb, ansi),
                "tty={is_tty} policy={policy:?} dumb={dumb} ansi={ansi}",
            );
        }
    }

    #[test]
    fn TEST_NON_TTY_FOLLOWS_POLICY() {
        let not_a_tty = TempNotATty::create();

        assert!(!stream_is_tty(not_a_tty.file()));

        with_env(&[], || {
            assert_eq!(ColourPolicy::Auto, colour_policy());
            assert!(!stream_supports_colour(not_a_tty.file()));
            assert_eq!("", seq_for_stream(FG_RED, not_a_tty.file()));
            assert_eq!("", super::bold_for(not_a_tty.file()));
            assert_eq!("", super::blinking_for(not_a_tty.file()));
            assert_eq!("", super::reset_for(not_a_tty.file()));
            assert!(!supports_colour_absent());
            assert_eq!("", seq_for_absent(FG_RED));
        });

        with_env(&[("WOAD_COLOUR", "always")], || {
            assert!(stream_supports_colour(not_a_tty.file()));
            assert_eq!(FG_RED, seq_for(FG_RED, not_a_tty.file()));
            assert_eq!(FG_RED, seq_for_stream(FG_RED, not_a_tty.file()));
            assert_eq!(BOLD, super::bold_for(not_a_tty.file()));
            assert_eq!(BLINKING, super::blinking_for(not_a_tty.file()));
            assert_eq!(RESET, super::reset_for(not_a_tty.file()));
            assert_eq!(FG_RED, super::fg_for(FG_RED, not_a_tty.file()));
            assert_eq!(crate::BG_BLACK, super::bg_for(crate::BG_BLACK, not_a_tty.file()));
            assert!(supports_colour(not_a_tty.file()));
            assert!(supports_colour_absent());
            assert_eq!(FG_RED, seq_for_absent(FG_RED));
            assert_eq!("", seq_for("", not_a_tty.file()));
        });

        with_env(&[("WOAD_COLOUR", "never")], || {
            assert!(!stream_supports_colour(not_a_tty.file()));
            assert_eq!("", seq_for(FG_RED, not_a_tty.file()));
            assert_eq!("", seq_for_absent(FG_RED));
        });
    }

    #[test]
    fn TEST_STREAM_IS_TTY_AGREES_WITH_IsTerminal() {
        let stdout = std::io::stdout();
        let stderr = std::io::stderr();
        let stdin = std::io::stdin();

        assert_eq!(stdout.is_terminal(), stream_is_tty(&stdout));
        assert_eq!(stderr.is_terminal(), stream_is_tty(&stderr));
        assert_eq!(stdin.is_terminal(), stream_is_tty(&stdin));
    }

    #[cfg(unix)]
    #[test]
    fn TEST_fd_is_tty_AGREES_WITH_stream_is_tty() {
        use std::os::fd::AsFd as _;

        let not_a_tty = TempNotATty::create();

        assert_eq!(stream_is_tty(not_a_tty.file()), super::fd_is_tty(not_a_tty.file()));

        with_env(&[("WOAD_COLOUR", "always")], || {
            assert!(super::fd_supports_colour(not_a_tty.file()));
            assert_eq!(FG_RED, super::seq_for_fd(FG_RED, not_a_tty.file().as_fd()));
            assert_eq!(
                seq_for(FG_RED, &not_a_tty.file().as_fd()),
                super::seq_for_fd(FG_RED, not_a_tty.file().as_fd()),
            );
        });

        with_env(&[], || {
            assert!(!super::fd_supports_colour(not_a_tty.file()));
            assert_eq!("", super::seq_for_fd(FG_RED, not_a_tty.file()));
        });
    }

    #[test]
    fn TEST_CONSOLE_SUPPORTS_ANSI_IS_TRUE() {
        assert!(console_supports_ansi());
        assert_eq!(console_supports_ansi(), console_supports_ansi());
    }

    #[test]
    fn TEST_WOAD_SET_CONSOLE_MODE_DOES_NOT_MOVE_POLICY() {
        with_env(&[("WOAD_SET_CONSOLE_MODE", "never")], || {
            assert!(console_supports_ansi());
            assert_eq!(ColourPolicy::Auto, colour_policy());
        });

        with_env(&[("WOAD_SET_CONSOLE_MODE", "banana")], || {
            assert!(console_supports_ansi());
            assert_eq!(ColourPolicy::Auto, colour_policy());
        });
    }

    #[test]
    fn TEST_OVERLAY_DOES_NOT_POISON_THE_CACHE() {
        let before = colour_policy();

        with_env(&[("WOAD_COLOUR", "always")], || {
            assert_eq!(ColourPolicy::Force, colour_policy());
        });

        assert_eq!(before, colour_policy());
    }

    #[test]
    fn TEST_REFRESH_KEEPS_A_STABLE_POLICY() {
        let first = colour_policy();

        refresh();

        assert_eq!(first, colour_policy());
    }

    #[test]
    fn TEST_POLICY_NAMES() {
        assert_eq!("auto", ColourPolicy::Auto.as_str());
        assert_eq!("force", ColourPolicy::Force.as_str());
        assert_eq!("inhibit", ColourPolicy::Inhibit.as_str());
        assert_eq!("auto", ColourPolicy::Auto.to_string());
    }
}
