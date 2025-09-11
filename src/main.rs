use std::process;
use minigrep;

fn main() {
    let args = minigrep::parse_args();

    let config = minigrep::Config::new(&args);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
