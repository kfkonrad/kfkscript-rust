use std::fs;
use std::error::Error;

mod parser;
mod token;
use parser::{parse, print_tokens};

fn main() -> Result<(), Box<dyn Error>> {
      let code: String = fs::read_to_string("example.kfkscript")?;
      let tokens = parse(code)?;
      print_tokens(tokens);
    Ok(())
}
