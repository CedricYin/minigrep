use minigrep::*;
use std::process;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
