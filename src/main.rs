//! Driver-code

use std::io::{Error, ErrorKind};
use std::{env, io};

mod iast;
mod tests;
mod uast;

fn main() -> Result<(), Error> {
    let mut args = env::args();
    if args.len() > 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments. Usage: uast [d|i|h]",
        ));
    }

    let f = match args.nth(1).unwrap_or_else(|| "d".to_string()).as_str() {
        "d" => uast::process_uast,
        "i" => iast::devanāgarī_to_iast,
        _ => {
            return Err(Error::new(ErrorKind::InvalidInput, "Usage: uast [d|i|h]"));
        }
    };

    loop {
        let mut l = String::new();

        match io::stdin().read_line(&mut l) {
            Err(e) => {
                return Err(e);
            }

            Ok(e) => {
                if e == 0 {
                    return Ok(());
                }

                println!(
                    "{}",
                    l.split_whitespace()
                        .map(|x| f(x.to_string()))
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
        };
    }
}
