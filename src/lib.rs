mod config;
pub use config::*;

mod search;
use search::*;

use std::error::Error;
use std::fs;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let vec = 
    if config.is_case {
        search_is_case(&config.query, &contents)
    } else {
        search_not_case(&config.query, &contents)
    };
    for v in vec {
        println!("{v}");
    }
    Ok(())
}