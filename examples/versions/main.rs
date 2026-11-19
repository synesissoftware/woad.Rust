use std::io as std_io;


fn main() {
    let stdout = std_io::stdout();

    println!(
        "{}v{}{} ({:#010x})",
        woad::seq_for(woad::FG_GREEN, &stdout),
        woad::version_string(),
        woad::reset_for(&stdout),
        woad::version(),
    );
}
