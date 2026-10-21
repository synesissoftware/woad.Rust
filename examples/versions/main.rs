fn main() {
    let stdout = std::io::stdout();

    println!(
        "{}v{}{} ({:#010x})",
        woad::seq_for(woad::FG_GREEN, &stdout),
        woad::version_string(),
        woad::reset_for(&stdout),
        woad::version(),
    );
}
