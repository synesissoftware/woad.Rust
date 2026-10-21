// Emits colour only when the destination warrants it.
//
//   cargo run --example colour_for
//   cargo run --example colour_for | cat
//   WOAD_COLOUR=always cargo run --example colour_for | cat
//   WOAD_COLOUR=never cargo run --example colour_for

fn main() {
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();

    println!(
        "policy: {}; stdout is{} a tty; stderr is{} a tty",
        woad::colour_policy(),
        if woad::stream_is_tty(&stdout) { "" } else { " not" },
        if woad::stream_is_tty(&stderr) { "" } else { " not" },
    );

    println!(
        "{}this line is green if, and only if, stdout warrants colour{}",
        woad::seq_for(woad::FG_GREEN, &stdout),
        woad::reset_for(&stdout),
    );

    println!(
        "{}this line is bold if, and only if, stdout warrants colour{}",
        woad::bold_for(&stdout),
        woad::reset_for(&stdout),
    );

    println!(
        "{}this line blinks if, and only if, stdout warrants colour{}",
        woad::blinking_for(&stdout),
        woad::reset_for(&stdout),
    );

    eprintln!(
        "{}{}this line is white-on-red if, and only if, stderr warrants colour{}",
        woad::bg_for(woad::BG_RED, &stderr),
        woad::fg_for(woad::FG_BRIGHT_WHITE, &stderr),
        woad::reset_for(&stderr),
    );

    println!(
        "{}this line is yellow if, and only if, stdout warrants colour{}",
        woad::fg_for(woad::FG_YELLOW, &stdout),
        woad::reset_for(&stdout),
    );

    #[cfg(unix)]
    {
        use std::os::fd::AsFd as _;

        println!(
            "{}this line is cyan if, and only if, stdout's descriptor warrants colour{}",
            woad::seq_for_fd(woad::FG_CYAN, stdout.as_fd()),
            woad::seq_for_fd(woad::RESET, stdout.as_fd()),
        );
    }
}
