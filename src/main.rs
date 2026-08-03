mod cli;
mod interactive;

fn main() {
    if let Err(error) = cli::run() {
        eprintln!("{error}");
        std::process::exit(error.exit_code());
    }
}
