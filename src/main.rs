mod cli;
mod interactive;
mod tui;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("{error}");
        std::process::exit(error.exit_code());
    }
}
