fn main() {
    if let Err(error) = epistem::cli::run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
