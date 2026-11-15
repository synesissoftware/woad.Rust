//! Portable colour policy.
//!
//! Cached environment inspection for the Unix half of the **woad** 0.3.0
//! colour contract. Windows virtual-terminal gating is not in this alpha.

use std::{
    cell::{
        self as std_cell,
        Cell,
    },
    env,
    ffi::{
        OsStr,
        OsString,
    },
    io::IsTerminal,
    sync::{
        Mutex,
        MutexGuard,
    },
};


/* /////////////////////////////////////////////////////////////////////////
 * enumerations
 */

/// Prevailing colour policy. Discriminants are the **woad** 0.3.0 values.
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[repr(i32)]
pub enum ColourPolicy {
    /// Colour depends on the writer. `TERM=dumb` suppresses colour on this
    /// path only.
    Auto = 0,

    /// Colour for every writer, including a non-terminal and `TERM=dumb`.
    Force = 1,

    /// Colour for no writer, including a terminal.
    Inhibit = 2,
}


/* /////////////////////////////////////////////////////////////////////////
 * constants
 */

const TRUEY : &[&str] = &["1", "always", "force", "on", "true", "yes"];

const FALSEY : &[&str] = &["0", "false", "never", "no", "none", "off"];


/* /////////////////////////////////////////////////////////////////////////
 * types and globals
 */

struct Environment {
    determined :   bool,
    policy :       ColourPolicy,
    term_is_dumb : bool,
}

/// Environment cache. Touched only while [`CACHE_GATE`] is held by the
/// thread.
struct Cache(std_cell::UnsafeCell<Environment>);

// SAFETY: every access goes through `with_environment`, which holds
// `CACHE_GATE`. Re-entry is same-thread only, so the mutable borrow does not
// alias another thread.
unsafe impl Sync for Cache {
}


/* /////////////////////////////////////////////////////////
 * globals
 */

/// Process-wide colour-policy cache.
///
/// `determined` starts `false`. The policy and the `TERM=dumb` flag are
/// placeholders until `ensure` fills them. `refresh` sets `determined`
/// back to `false` and does not itself re-read.
///
/// The value sits in an `UnsafeCell` so `with_environment` can borrow it
/// mutably while `CACHE_GATE` is held, including when this thread
/// re-enters. Every access goes through that function.
static CACHE : Cache = Cache(std_cell::UnsafeCell::new(Environment {
    determined :   false,
    policy :       ColourPolicy::Auto,
    term_is_dumb : false,
}));


/// Mutex that gives one thread at a time exclusive access to `CACHE`.
///
/// The mutex carries no data. The cache lives in `CACHE`, and every access
/// goes through `with_environment` while this lock is held.
///
/// `CACHE_HELD` is not what this guards. It is a thread-local flag so the
/// thread that already holds this lock can re-enter without locking again.
/// Other threads wait here.
static CACHE_GATE : Mutex<()> = Mutex::new(());


thread_local! {
    /// Thread-local flag: whether this thread already holds `CACHE_GATE`.
    ///
    /// `false` means it does not, so `with_environment` locks `CACHE_GATE`
    /// and then stores `true`. `true` means it does, so a nested call uses
    /// `CACHE` without locking again.
    ///
    /// `LocalKey::get` and `LocalKey::set` are implemented for `Cell<T>`.
    /// A plain `bool` has neither, so the flag is a `Cell<bool>`.
    static CACHE_HELD : Cell<bool> = const { Cell::new(false) };
}


/* /////////////////////////////////////////////////////////////////////////
 * helper functions
 */

/// Fills `env` from the process environment when not yet determined.
fn ensure(env : &mut Environment) {
    if !env.determined {
        env.policy = policy_from_environment();
        env.term_is_dumb = term_is_dumb_from_environment();
        env.determined = true;
    }
}


/// Whether `value` equals `candidate`, ignoring ASCII case.
///
/// A non-Unicode `OsStr` does not match.
fn eq_ignore_ascii(
    value : &OsStr,
    candidate : &str,
) -> bool {
    match value.to_str() {
        Some(text) => text.eq_ignore_ascii_case(candidate),
        None => false,
    }
}


/// Whether `value` equals any entry in `candidates`, ignoring ASCII case.
fn is_listed(
    value : &OsStr,
    candidates : &[&str],
) -> bool {
    candidates.iter().any(|candidate| eq_ignore_ascii(value, candidate))
}


/// Locks `CACHE_GATE` and recovers a poisoned lock.
fn lock_gate() -> MutexGuard<'static, ()> {
    CACHE_GATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}


/// Value of `name` when the variable is set and not empty.
fn nonempty(name : &str) -> Option<OsString> {
    let value = env::var_os(name)?;

    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}


/// Colour policy from the current environment, ignoring the cache.
fn policy_from_environment() -> ColourPolicy {
    if let Some(value) = nonempty("WOAD_COLOUR").or_else(|| nonempty("WOAD_COLOR")) {
        if is_listed(&value, TRUEY) {
            return ColourPolicy::Force;
        }

        if is_listed(&value, FALSEY) {
            return ColourPolicy::Inhibit;
        }

        return ColourPolicy::Auto;
    }

    if nonempty("NO_COLOR").is_some() {
        return ColourPolicy::Inhibit;
    }

    if let Some(value) = nonempty("CLICOLOR_FORCE") {
        if value != OsStr::new("0") {
            return ColourPolicy::Force;
        }
    }

    if let Some(value) = nonempty("FORCE_COLOR") {
        if value != OsStr::new("0") && !eq_ignore_ascii(&value, "false") {
            return ColourPolicy::Force;
        }
    }

    if let Some(value) = nonempty("CLICOLOR") {
        if value == OsStr::new("0") {
            return ColourPolicy::Inhibit;
        }
    }

    ColourPolicy::Auto
}


/// Whether colour should be emitted.
///
/// [`ColourPolicy::Force`] is always yes. [`ColourPolicy::Inhibit`] is
/// always no. [`ColourPolicy::Auto`] is yes only when `is_terminal` is
/// true and `TERM` is not `dumb`. Calls `ensure` first.
fn supports_for(
    env : &mut Environment,
    is_terminal : bool,
) -> bool {
    ensure(env);

    match env.policy {
        ColourPolicy::Auto => is_terminal && !env.term_is_dumb,
        ColourPolicy::Force => true,
        ColourPolicy::Inhibit => false,
    }
}


/// Whether `TERM` equals `dumb`, ignoring ASCII case.
///
/// An absent or empty `TERM` is not dumb.
fn term_is_dumb_from_environment() -> bool {
    match nonempty("TERM") {
        Some(value) => eq_ignore_ascii(&value, "dumb"),
        None => false,
    }
}


/// Runs `body` with exclusive access to the process-wide environment cache.
///
/// `CACHE_GATE` is a `Mutex`. The same thread cannot lock it twice, and the
/// public queries call this function while a caller may already be inside
/// it. `CACHE_HELD` records that for this thread only:
///
/// * `false` means this thread does not hold `CACHE_GATE`. The call locks
///   `CACHE_GATE`, sets `CACHE_HELD` to `true`, and runs `body`;
/// * `true` means this thread already holds `CACHE_GATE`. The call runs
///   `body` without locking again;
///
/// `ClearCacheHeld` sets `CACHE_HELD` back to `false` when the outer call
/// returns, including after a panic, and it does so before the mutex guard
/// drops.
/// Other threads never read `CACHE_HELD`. They wait on `CACHE_GATE`.
///
/// This is one lock for the outer call, not a recursive mutex. Nested
/// calls do not lock `CACHE_GATE` again and do not count entries. A
/// direct lock of `CACHE_GATE` on this thread still waits.
fn with_environment<R>(body : impl FnOnce(&mut Environment) -> R) -> R {
    struct ClearCacheHeld;

    impl Drop for ClearCacheHeld {
        fn drop(&mut self) {
            CACHE_HELD.set(false);
        }
    }

    if CACHE_HELD.get() {
        // SAFETY: this thread already holds `CACHE_GATE` (see `Cache`).
        //
        // Now we may invoke safely the `body` function.
        body(unsafe { &mut *CACHE.0.get() })
    } else {
        let _guard = lock_gate();

        CACHE_HELD.set(true);

        let _reset = ClearCacheHeld;

        // SAFETY: `CACHE_GATE` is held, and `CACHE_HELD` blocks other entries on
        // this thread from taking a second guard. Other threads wait
        // on `CACHE_GATE`.
        //
        // Now we may invoke safely the `body` function.
        body(unsafe { &mut *CACHE.0.get() })
    }
}


/* /////////////////////////////////////////////////////////////////////////
 * API functions
 */

/// Prevailing colour policy, cached until [`refresh`].
///
/// Returns [`ColourPolicy::Auto`] (0), [`ColourPolicy::Force`] (1), or
/// [`ColourPolicy::Inhibit`] (2). An unset or empty variable is absent.
/// Values are not trimmed.
///
/// Precedence:
///
/// * non-empty `WOAD_COLOUR`, else non-empty `WOAD_COLOR` — truey forces,
///   falsey inhibits, and anything else (including `auto`) selects auto
///   and does not fall through;
/// * non-empty `NO_COLOR` inhibits, whatever its text is;
/// * non-empty `CLICOLOR_FORCE` other than the exact bytes `0` forces;
/// * non-empty `FORCE_COLOR` other than exact `0` or `false` (any
///   capitalisation) forces;
/// * `CLICOLOR` equal to the exact bytes `0` inhibits;
/// * otherwise auto.
///
/// Truey: `1`, `always`, `force`, `on`, `true`, `yes`. Falsey: `0`,
/// `false`, `never`, `no`, `none`, `off`. Both lists are
/// case-insensitive. `CLICOLOR` and `CLICOLOR_FORCE` compare `0`
/// case-sensitively.
pub fn colour_policy() -> ColourPolicy {
    with_environment(|env| {
        ensure(env);
        env.policy
    })
}


/// Whether `writer` is attached to a terminal.
///
/// This is the host check only. It does not apply the colour policy.
pub fn is_tty<W>(writer : &W) -> bool
where
    W : IsTerminal + ?Sized,
{
    writer.is_terminal()
}


/// Drops the cached policy and the cached `TERM=dumb` flag.
///
/// The next query reads the environment again. On Unix there is no
/// console cache to drop.
pub fn refresh() {
    with_environment(|env| {
        env.determined = false;
    });
}


/// `seq` when [`supports_colour`] is true for `writer`, otherwise `""`.
///
/// An empty `seq` yields `""`.
pub fn seq_for<'seq, W>(
    seq : &'seq str,
    writer : &W,
) -> &'seq str
where
    W : IsTerminal + ?Sized,
{
    if seq.is_empty() || !supports_colour(writer) {
        ""
    } else {
        seq
    }
}


/// Whether colour should be emitted for `writer`.
///
/// [`ColourPolicy::Force`] is yes for every writer.
/// [`ColourPolicy::Inhibit`] is no for every writer.
/// [`ColourPolicy::Auto`] is yes only when `writer` is a terminal and
/// `TERM` is not `dumb` (any capitalisation).
///
/// On Unix a terminal is ANSI-capable. Windows virtual-terminal gating and
/// mintty recognition are not implemented in this alpha; a Windows terminal
/// is treated with the Unix rule.
pub fn supports_colour<W>(writer : &W) -> bool
where
    W : IsTerminal + ?Sized,
{
    let is_terminal = writer.is_terminal();

    with_environment(|env| supports_for(env, is_terminal))
}


#[cfg(all(test, unix))]
mod tests {
    #![allow(non_snake_case)]

    use std::{
        ffi as std_ffi,
        fs::{
            File,
            OpenOptions,
        },
        io as std_io,
        os::{
            fd::FromRawFd,
            unix::{
                ffi::OsStrExt,
                fs::OpenOptionsExt,
            },
        },
        sync::OnceLock,
    };

    use super::*;


    const MANAGED_VARS : &[&str] = &[
        "CLICOLOR",
        "CLICOLOR_FORCE",
        "FORCE_COLOR",
        "NO_COLOR",
        "TERM",
        "WOAD_COLOR",
        "WOAD_COLOUR",
    ];


    struct SavedVar {
        name :  &'static str,
        value : Option<OsString>,
    }


    fn capture_managed() -> Vec<SavedVar> {
        MANAGED_VARS
            .iter()
            .map(|name| {
                SavedVar {
                    name,
                    value : env::var_os(name),
                }
            })
            .collect()
    }


    fn apply_saved(saved : &[SavedVar]) {
        for item in saved {
            match &item.value {
                Some(value) => unsafe { env::set_var(item.name, value) },
                None => unsafe { env::remove_var(item.name) },
            }
        }
    }


    fn assign(
        name : &str,
        value : &str,
    ) {
        unsafe { env::set_var(name, value) };
    }


    fn clear_managed() {
        for name in MANAGED_VARS {
            unsafe { env::remove_var(name) };
        }
    }


    /// Holds the policy gate across environment edits so parallel tests
    /// cannot observe a half-applied row.
    fn with_managed<R>(
        assignments : &[(&str, &str)],
        body : impl FnOnce() -> R,
    ) -> R {
        with_environment(|_| {
            let saved = capture_managed();
            clear_managed();

            for (name, value) in assignments {
                assign(name, value);
            }

            refresh();

            let result = body();

            apply_saved(&saved);
            refresh();

            result
        })
    }


    struct Endpoints {
        terminal : File,
        _master :  File,
        detached : File,
    }


    fn endpoints() -> &'static Endpoints {
        static ENDPOINTS : OnceLock<Endpoints> = OnceLock::new();

        ENDPOINTS.get_or_init(|| {
            let (terminal, master) = open_pty_slave();
            let detached = File::open("/dev/null").expect("open /dev/null");

            assert!(terminal.is_terminal());
            assert!(!detached.is_terminal());

            Endpoints {
                terminal,
                _master : master,
                detached,
            }
        })
    }


    fn open_pty_slave() -> (File, File) {
        #[cfg(any(target_os = "android", target_os = "linux"))]
        const O_NOCTTY : i32 = 0x100;

        #[cfg(target_os = "macos")]
        const O_NOCTTY : i32 = 0x2_0000;

        #[cfg(not(any(target_os = "android", target_os = "linux", target_os = "macos")))]
        const O_NOCTTY : i32 = 0x8000;

        const O_RDWR : i32 = 2;

        extern "C" {
            fn close(fd : i32) -> i32;
            fn grantpt(fd : i32) -> i32;
            fn posix_openpt(flags : i32) -> i32;
            fn ptsname(fd : i32) -> *mut std_ffi::c_char;
            fn unlockpt(fd : i32) -> i32;
        }

        unsafe {
            let master = posix_openpt(O_RDWR | O_NOCTTY);
            assert!(master >= 0, "posix_openpt: {}", std_io::Error::last_os_error());

            if grantpt(master) != 0 || unlockpt(master) != 0 {
                let err = std_io::Error::last_os_error();
                close(master);
                panic!("grantpt/unlockpt: {err}");
            }

            let name = ptsname(master);
            assert!(!name.is_null(), "ptsname");

            let path = std_ffi::OsStr::from_bytes(std_ffi::CStr::from_ptr(name).to_bytes());
            let slave = match OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(O_NOCTTY)
                .open(path)
            {
                Ok(file) => file,
                Err(err) => {
                    close(master);
                    panic!("open slave: {err}");
                },
            };

            (slave, File::from_raw_fd(master))
        }
    }


    fn writer(terminal : bool) -> &'static File {
        let endpoints = endpoints();

        if terminal {
            &endpoints.terminal
        } else {
            &endpoints.detached
        }
    }


    fn assert_row(
        row : u32,
        assignments : &[(&str, &str)],
        terminal : bool,
        policy : ColourPolicy,
        colour : bool,
    ) {
        with_managed(assignments, || {
            let attached = writer(terminal);

            assert_eq!(is_tty(attached), terminal, "row {row} attachment");
            assert_eq!(colour_policy(), policy, "row {row} policy");
            assert_eq!(supports_colour(attached), colour, "row {row} colour");

            let seq = seq_for(crate::FG_RED, attached);
            if colour {
                assert_eq!(seq, crate::FG_RED, "row {row} sequence");
            } else {
                assert_eq!(seq, "", "row {row} sequence");
            }
        });
    }


    #[test]
    fn TEST_ROW_1_NONE_TERMINAL() {
        assert_row(1, &[], true, ColourPolicy::Auto, true);
    }

    #[test]
    fn TEST_ROW_2_NONE_DETACHED() {
        assert_row(2, &[], false, ColourPolicy::Auto, false);
    }

    #[test]
    fn TEST_ROW_3_TERM_DUMB() {
        assert_row(3, &[("TERM", "dumb")], true, ColourPolicy::Auto, false);
    }

    #[test]
    fn TEST_ROW_4_TERM_DUMB_UPPER() {
        assert_row(4, &[("TERM", "DUMB")], true, ColourPolicy::Auto, false);
    }

    #[test]
    fn TEST_ROW_5_WOAD_COLOUR_1() {
        assert_row(5, &[("WOAD_COLOUR", "1")], false, ColourPolicy::Force, true);
    }

    #[test]
    fn TEST_ROW_6_WOAD_COLOR_ALWAYS() {
        assert_row(6, &[("WOAD_COLOR", "always")], false, ColourPolicy::Force, true);
    }

    #[test]
    fn TEST_ROW_7_WOAD_COLOUR_NEVER() {
        assert_row(7, &[("WOAD_COLOUR", "never")], true, ColourPolicy::Inhibit, false);
    }

    #[test]
    fn TEST_ROW_8_WOAD_COLOUR_AUTO_TERMINAL() {
        assert_row(8, &[("WOAD_COLOUR", "auto")], true, ColourPolicy::Auto, true);
    }

    #[test]
    fn TEST_ROW_9_WOAD_COLOUR_AUTO_DETACHED() {
        assert_row(9, &[("WOAD_COLOUR", "auto")], false, ColourPolicy::Auto, false);
    }

    #[test]
    fn TEST_ROW_10_WOAD_COLOUR_PURPLE() {
        assert_row(10, &[("WOAD_COLOUR", "purple")], true, ColourPolicy::Auto, true);
    }

    #[test]
    fn TEST_ROW_11_NO_COLOR_BEATS_CLICOLOR_FORCE() {
        assert_row(
            11,
            &[("CLICOLOR_FORCE", "1"), ("NO_COLOR", "1")],
            true,
            ColourPolicy::Inhibit,
            false,
        );
    }

    #[test]
    fn TEST_ROW_12_CLICOLOR_FORCE() {
        assert_row(12, &[("CLICOLOR_FORCE", "1")], false, ColourPolicy::Force, true);
    }

    #[test]
    fn TEST_ROW_13_CLICOLOR_FORCE_0_THEN_CLICOLOR_0() {
        assert_row(
            13,
            &[("CLICOLOR", "0"), ("CLICOLOR_FORCE", "0")],
            true,
            ColourPolicy::Inhibit,
            false,
        );
    }

    #[test]
    fn TEST_ROW_14_FORCE_COLOR() {
        assert_row(14, &[("FORCE_COLOR", "1")], false, ColourPolicy::Force, true);
    }

    #[test]
    fn TEST_ROW_15_FORCE_COLOR_FALSE_THEN_CLICOLOR_0() {
        assert_row(
            15,
            &[("CLICOLOR", "0"), ("FORCE_COLOR", "false")],
            true,
            ColourPolicy::Inhibit,
            false,
        );
    }

    #[test]
    fn TEST_ROW_16_FORCE_COLOR_FALSE_UPPER() {
        assert_row(16, &[("FORCE_COLOR", "FALSE")], false, ColourPolicy::Auto, false);
    }

    #[test]
    fn TEST_ROW_17_FORCE_COLOR_0() {
        assert_row(17, &[("FORCE_COLOR", "0")], true, ColourPolicy::Auto, true);
    }

    #[test]
    fn TEST_ROW_18_CLICOLOR_0() {
        assert_row(18, &[("CLICOLOR", "0")], true, ColourPolicy::Inhibit, false);
    }

    #[test]
    fn TEST_ROW_19_CLICOLOR_1() {
        assert_row(19, &[("CLICOLOR", "1")], true, ColourPolicy::Auto, true);
    }

    #[test]
    fn TEST_ROW_20_EMPTY_WOAD_COLOUR_FALLS_THROUGH() {
        assert_row(
            20,
            &[("NO_COLOR", "1"), ("WOAD_COLOUR", "")],
            true,
            ColourPolicy::Inhibit,
            false,
        );
    }

    #[test]
    fn TEST_ROW_21_EMPTY_NO_COLOR_IS_ABSENT() {
        assert_row(21, &[("NO_COLOR", "")], true, ColourPolicy::Auto, true);
    }

    #[test]
    fn TEST_ROW_22_REFRESH_DROPS_THE_CACHE() {
        with_managed(&[], || {
            assign("WOAD_COLOUR", "1");
            refresh();

            let attached = writer(true);
            assert_eq!(colour_policy(), ColourPolicy::Force, "row 22 before refresh");
            assert!(supports_colour(attached), "row 22 before refresh");

            unsafe { env::remove_var("WOAD_COLOUR") };
            assign("NO_COLOR", "1");
            refresh();

            assert_eq!(colour_policy(), ColourPolicy::Inhibit, "row 22 after refresh");
            assert!(!supports_colour(attached), "row 22 after refresh");
            assert_eq!(seq_for(crate::BOLD, attached), "");
            assert!(is_tty(attached), "row 22 attachment stays a terminal");
        });
    }

    #[test]
    fn TEST_SEQ_FOR_EMPTY_IS_EMPTY() {
        with_managed(&[("WOAD_COLOUR", "1")], || {
            assert_eq!(seq_for("", writer(false)), "");
        });
    }
}
