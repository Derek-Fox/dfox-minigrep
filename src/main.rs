use minigrep;

fn main() {
    let args = minigrep::parse_args();
    let config = minigrep::Config::new(&args);
    minigrep::run(config);
}
