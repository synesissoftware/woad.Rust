//! Crate version, including the packed **woad** `WOAD_VER` word.

/// Major component of [`version`].
pub const VERSION_MAJOR : u32 = 0;

/// Minor component of [`version`].
pub const VERSION_MINOR : u32 = 1;

/// Patch component of [`version`].
pub const VERSION_PATCH : u32 = 0;

/// Release-status byte of [`version`].
///
/// `0x42` is alpha 2. `0xFF` is a numbered release. The layout matches
/// **woad** `WOAD_VER`: bits 24-31 major, 16-23 minor, 8-15 patch, and
/// 0-7 release status.
pub const VERSION_RELSTATUS : u32 = 0x42;

/// Crate version, SemVer, including the prerelease label.
pub const VERSION : &str = "0.1.0";

/// Packed **woad** version, in the `WOAD_VER` layout.
///
/// ```
/// assert_eq!(0x0001_0042, woad::version());
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
        assert_eq!("0.1.0", crate::VERSION);
        assert_eq!(crate::VERSION, crate::version_string());
        assert_eq!(crate::VERSION, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn TEST_VERSION_PACKED() {
        assert_eq!(0, crate::VERSION_MAJOR);
        assert_eq!(1, crate::VERSION_MINOR);
        assert_eq!(0, crate::VERSION_PATCH);
        assert_eq!(0x42, crate::VERSION_RELSTATUS);
        assert_eq!(0x0001_0042, crate::version());
    }
}
