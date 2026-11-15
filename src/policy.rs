//! Portable colour policy.
//!
//! Cached environment inspection for the Unix half of the **woad** 0.3.0
//! colour contract. Windows virtual-terminal gating is not in this alpha.

use std::{
    cell::Cell,
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


/// Colour depends on the writer. `TERM=dumb` suppresses colour on this
/// path only.
pub const COLOUR_POLICY_AUTO : i32 = 0;

/// Colour for every writer, including a non-terminal and `TERM=dumb`.
pub const COLOUR_POLICY_FORCE : i32 = 1;

/// Colour for no writer, including a terminal.
pub const COLOUR_POLICY_INHIBIT : i32 = 2;


const TRUEY : &[&str] = &["1", "always", "force", "on", "true", "yes"];

const FALSEY : &[&str] = &["0", "false", "never", "no", "none", "off"];

struct Environment {
    determined :   bool,
    policy :       i32,
    term_is_dumb : bool,
}


/// Environment cache. Touched only while [`GATE`] is held by this thread.
struct Cache(std::cell::UnsafeCell<Environment>);

// SAFETY: every access goes through `with_environment`, which holds
// `GATE`. Re-entry is same-thread only, so the mutable borrow does not
// alias another thread.
unsafe impl Sync for Cache {
}


static CACHE : Cache = Cache(std::cell::UnsafeCell::new(Environment {
    determined :   false,
    policy :       COLOUR_POLICY_AUTO,
    term_is_dumb : false,
}));

static GATE : Mutex<()> = Mutex::new(());

thread_local! {
    static DEPTH : Cell<u32> = const { Cell::new(0) };
}


struct DepthReset;

impl Drop for DepthReset {
    fn drop(&mut self) {
        DEPTH.with(|depth| {
            depth.set(0);
        });
    }
}


fn lock_gate() -> MutexGuard<'static, ()> {
    GATE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}


fn with_environment<R>(body : impl FnOnce(&mut Environment) -> R) -> R {
    DEPTH.with(|depth| {
        if depth.get() > 0 {
            // SAFETY: this thread already holds `GATE` (see `Cache`).
            body(unsafe { &mut *CACHE.0.get() })
        } else {
            let _gate = lock_gate();
            depth.set(1);
            let _reset = DepthReset;
            // SAFETY: `GATE` is held, and `DEPTH` blocks other entries on
            // this thread from taking a second guard. Other threads wait
            // on `GATE`.
            body(unsafe { &mut *CACHE.0.get() })
        }
    })
}


fn nonempty(name : &str) -> Option<OsString> {
    let value = env::var_os(name)?;

    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}


fn eq_ignore_ascii(
    value : &OsStr,
    candidate : &str,
) -> bool {
    match value.to_str() {
        Some(text) => text.eq_ignore_ascii_case(candidate),
        None => false,
    }
}


fn is_listed(
    value : &OsStr,
    candidates : &[&str],
) -> bool {
    candidates.iter().any(|candidate| eq_ignore_ascii(value, candidate))
}


fn policy_from_environment() -> i32 {
    let own = nonempty("WOAD_COLOUR").or_else(|| nonempty("WOAD_COLOR"));

    if let Some(value) = own {
        if is_listed(&value, TRUEY) {
            return COLOUR_POLICY_FORCE;
        }

        if is_listed(&value, FALSEY) {
            return COLOUR_POLICY_INHIBIT;
        }

        return COLOUR_POLICY_AUTO;
    }

    if nonempty("NO_COLOR").is_some() {
        return COLOUR_POLICY_INHIBIT;
    }

    if let Some(value) = nonempty("CLICOLOR_FORCE") {
        if value != OsStr::new("0") {
            return COLOUR_POLICY_FORCE;
        }
    }

    if let Some(value) = nonempty("FORCE_COLOR") {
        if value != OsStr::new("0") && !eq_ignore_ascii(&value, "false") {
            return COLOUR_POLICY_FORCE;
        }
    }

    if let Some(value) = nonempty("CLICOLOR") {
        if value == OsStr::new("0") {
            return COLOUR_POLICY_INHIBIT;
        }
    }

    COLOUR_POLICY_AUTO
}


fn term_is_dumb_from_environment() -> bool {
    match nonempty("TERM") {
        Some(value) => eq_ignore_ascii(&value, "dumb"),
        None => false,
    }
}


fn ensure(environment : &mut Environment) {
    if !environment.determined {
        environment.policy = policy_from_environment();
        environment.term_is_dumb = term_is_dumb_from_environment();
        environment.determined = true;
    }
}


fn supports_for(
    environment : &mut Environment,
    is_terminal : bool,
) -> bool {
    ensure(environment);

    match environment.policy {
        COLOUR_POLICY_FORCE => true,
        COLOUR_POLICY_INHIBIT => false,
        _ => is_terminal && !environment.term_is_dumb,
    }
}


/// Prevailing colour policy, cached until [`refresh`].
///
/// Returns [`COLOUR_POLICY_AUTO`] (0), [`COLOUR_POLICY_FORCE`] (1), or
/// [`COLOUR_POLICY_INHIBIT`] (2). An unset or empty variable is absent.
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
pub fn colour_policy() -> i32 {
    with_environment(|environment| {
        ensure(environment);
        environment.policy
    })
}


/// Drops the cached policy and the cached `TERM=dumb` flag.
///
/// The next query reads the environment again. On Unix there is no
/// console cache to drop.
pub fn refresh() {
    with_environment(|environment| {
        environment.determined = false;
    });
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


/// Whether colour should be emitted for `writer`.
///
/// [`COLOUR_POLICY_FORCE`] is yes for every writer.
/// [`COLOUR_POLICY_INHIBIT`] is no for every writer.
/// [`COLOUR_POLICY_AUTO`] is yes only when `writer` is a terminal and
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

    with_environment(|environment| supports_for(environment, is_terminal))
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


#[cfg(all(test, unix))]
mod tests {
    #![allow(non_snake_case)]

    use std::{
        fs::File,
        os::fd::FromRawFd,
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
            fn open(
                path : *const std::ffi::c_char,
                flags : i32,
            ) -> i32;
            fn posix_openpt(flags : i32) -> i32;
            fn ptsname(fd : i32) -> *mut std::ffi::c_char;
            fn unlockpt(fd : i32) -> i32;
        }

        unsafe {
            let master = posix_openpt(O_RDWR | O_NOCTTY);
            assert!(master >= 0, "posix_openpt: {}", std::io::Error::last_os_error());

            if grantpt(master) != 0 || unlockpt(master) != 0 {
                let err = std::io::Error::last_os_error();
                close(master);
                panic!("grantpt/unlockpt: {err}");
            }

            let name = ptsname(master);
            assert!(!name.is_null(), "ptsname");

            let slave = open(name, O_RDWR | O_NOCTTY);
            if slave < 0 {
                let err = std::io::Error::last_os_error();
                close(master);
                panic!("open slave: {err}");
            }

            (File::from_raw_fd(slave), File::from_raw_fd(master))
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
        policy : i32,
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
        assert_row(1, &[], true, COLOUR_POLICY_AUTO, true);
    }

    #[test]
    fn TEST_ROW_2_NONE_DETACHED() {
        assert_row(2, &[], false, COLOUR_POLICY_AUTO, false);
    }

    #[test]
    fn TEST_ROW_3_TERM_DUMB() {
        assert_row(3, &[("TERM", "dumb")], true, COLOUR_POLICY_AUTO, false);
    }

    #[test]
    fn TEST_ROW_4_TERM_DUMB_UPPER() {
        assert_row(4, &[("TERM", "DUMB")], true, COLOUR_POLICY_AUTO, false);
    }

    #[test]
    fn TEST_ROW_5_WOAD_COLOUR_1() {
        assert_row(5, &[("WOAD_COLOUR", "1")], false, COLOUR_POLICY_FORCE, true);
    }

    #[test]
    fn TEST_ROW_6_WOAD_COLOR_ALWAYS() {
        assert_row(6, &[("WOAD_COLOR", "always")], false, COLOUR_POLICY_FORCE, true);
    }

    #[test]
    fn TEST_ROW_7_WOAD_COLOUR_NEVER() {
        assert_row(7, &[("WOAD_COLOUR", "never")], true, COLOUR_POLICY_INHIBIT, false);
    }

    #[test]
    fn TEST_ROW_8_WOAD_COLOUR_AUTO_TERMINAL() {
        assert_row(8, &[("WOAD_COLOUR", "auto")], true, COLOUR_POLICY_AUTO, true);
    }

    #[test]
    fn TEST_ROW_9_WOAD_COLOUR_AUTO_DETACHED() {
        assert_row(9, &[("WOAD_COLOUR", "auto")], false, COLOUR_POLICY_AUTO, false);
    }

    #[test]
    fn TEST_ROW_10_WOAD_COLOUR_PURPLE() {
        assert_row(10, &[("WOAD_COLOUR", "purple")], true, COLOUR_POLICY_AUTO, true);
    }

    #[test]
    fn TEST_ROW_11_NO_COLOR_BEATS_CLICOLOR_FORCE() {
        assert_row(
            11,
            &[("CLICOLOR_FORCE", "1"), ("NO_COLOR", "1")],
            true,
            COLOUR_POLICY_INHIBIT,
            false,
        );
    }

    #[test]
    fn TEST_ROW_12_CLICOLOR_FORCE() {
        assert_row(12, &[("CLICOLOR_FORCE", "1")], false, COLOUR_POLICY_FORCE, true);
    }

    #[test]
    fn TEST_ROW_13_CLICOLOR_FORCE_0_THEN_CLICOLOR_0() {
        assert_row(
            13,
            &[("CLICOLOR", "0"), ("CLICOLOR_FORCE", "0")],
            true,
            COLOUR_POLICY_INHIBIT,
            false,
        );
    }

    #[test]
    fn TEST_ROW_14_FORCE_COLOR() {
        assert_row(14, &[("FORCE_COLOR", "1")], false, COLOUR_POLICY_FORCE, true);
    }

    #[test]
    fn TEST_ROW_15_FORCE_COLOR_FALSE_THEN_CLICOLOR_0() {
        assert_row(
            15,
            &[("CLICOLOR", "0"), ("FORCE_COLOR", "false")],
            true,
            COLOUR_POLICY_INHIBIT,
            false,
        );
    }

    #[test]
    fn TEST_ROW_16_FORCE_COLOR_FALSE_UPPER() {
        assert_row(16, &[("FORCE_COLOR", "FALSE")], false, COLOUR_POLICY_AUTO, false);
    }

    #[test]
    fn TEST_ROW_17_FORCE_COLOR_0() {
        assert_row(17, &[("FORCE_COLOR", "0")], true, COLOUR_POLICY_AUTO, true);
    }

    #[test]
    fn TEST_ROW_18_CLICOLOR_0() {
        assert_row(18, &[("CLICOLOR", "0")], true, COLOUR_POLICY_INHIBIT, false);
    }

    #[test]
    fn TEST_ROW_19_CLICOLOR_1() {
        assert_row(19, &[("CLICOLOR", "1")], true, COLOUR_POLICY_AUTO, true);
    }

    #[test]
    fn TEST_ROW_20_EMPTY_WOAD_COLOUR_FALLS_THROUGH() {
        assert_row(
            20,
            &[("NO_COLOR", "1"), ("WOAD_COLOUR", "")],
            true,
            COLOUR_POLICY_INHIBIT,
            false,
        );
    }

    #[test]
    fn TEST_ROW_21_EMPTY_NO_COLOR_IS_ABSENT() {
        assert_row(21, &[("NO_COLOR", "")], true, COLOUR_POLICY_AUTO, true);
    }

    #[test]
    fn TEST_ROW_22_REFRESH_DROPS_THE_CACHE() {
        with_managed(&[], || {
            assign("WOAD_COLOUR", "1");
            refresh();

            let attached = writer(true);
            assert_eq!(colour_policy(), COLOUR_POLICY_FORCE, "row 22 before refresh");
            assert!(supports_colour(attached), "row 22 before refresh");

            unsafe { env::remove_var("WOAD_COLOUR") };
            assign("NO_COLOR", "1");
            refresh();

            assert_eq!(colour_policy(), COLOUR_POLICY_INHIBIT, "row 22 after refresh");
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
