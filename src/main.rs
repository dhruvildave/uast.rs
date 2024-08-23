//! Driver-code

use std::process::ExitCode;
use std::{env, io};

mod iast;
mod tests;
mod uast;

fn main() -> ExitCode {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments. Usage: uast d|i|h");
        return ExitCode::FAILURE;
    }

    let devanāgarī_mode = match args.nth(1).unwrap().as_str() {
        "d" => true,
        "i" => false,
        _ => {
            eprintln!("Usage: uast d|i|h");
            return ExitCode::FAILURE;
        }
    };

    loop {
        let mut l = String::new();

        match io::stdin().read_line(&mut l) {
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }

            Ok(e) => {
                if e == 0 {
                    return ExitCode::SUCCESS;
                }

                println!(
                    "{}",
                    if devanāgarī_mode {
                        l.split_whitespace()
                            .map(|x| uast::process_uast(x.to_string()))
                            .collect::<Vec<String>>()
                            .join(" ")
                    } else {
                        l.split_whitespace()
                            .map(|x| iast::devanāgarī_to_iast(x.to_string()))
                            .collect::<Vec<String>>()
                            .join(" ")
                    }
                );
            }
        };
    }
}
