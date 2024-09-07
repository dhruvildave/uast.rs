//! Driver-code

use std::{env, io};

mod iast;
mod tests;
mod uast;

fn main() -> Result<(), String> {
    let mut args = env::args();
    if args.len() > 2 {
        return Err("Invalid number of arguments. Usage: uast [d|i|h]".to_string());
    }

    let f = match args.nth(1).unwrap_or_else(|| "d".to_string()).as_str() {
        "d" => uast::process_uast,
        "i" => iast::devanāgarī_to_iast,
        _ => {
            return Err("Usage: uast [d|i|h]".to_string());
        }
    };

    loop {
        let mut l = String::new();

        match io::stdin().read_line(&mut l) {
            Err(e) => {
                return Err(e.to_string());
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
