pub use std::env;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("[error] query is empty\nUsage: cargo run -- <query> <file_path>\n"),
        };
        let filename = match args.next() {
            Some(f) => f,
            None => return Err("[error] filename is empty\nUsage: cargo run -- <query> <file_path>\n"),
        };
        if let Some(_) = args.next() {
            eprintln!("[warning] too many args\nUsage: cargo run -- <query> <file_path>\n");
        }
        let is_case = env::var("IS_CASE").is_ok();
        Ok(Config { query, filename, is_case })
    }
}