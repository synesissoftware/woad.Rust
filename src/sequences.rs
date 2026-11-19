//! Fixed SGR sequences.
//!
//! These bytes do not consult the environment. The crate root re-exports
//! them.

/// Reset all attributes.
pub const RESET : &str = "\x1b[0m";

/// Bold (SGR 1).
pub const BOLD : &str = "\x1b[1m";

/// Blinking (SGR 5).
pub const BLINKING : &str = "\x1b[5m";


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
