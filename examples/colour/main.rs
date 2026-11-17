fn main() {
    let stdout = std::io::stdout();
    let open = woad::seq_for(woad::FG_GREEN, &stdout);
    let close = woad::seq_for(woad::RESET, &stdout);

    println!("{open}ok{close}");
}
