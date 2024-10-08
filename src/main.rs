//! Driver-code

use std::env::consts::{ARCH, OS};
use std::io::{Error, ErrorKind};
use std::{env, io};

mod gu;
mod iast;
mod tests;
mod uast;

fn main() -> Result<(), Error> {
    let mut args = env::args();
    if args.len() > 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments. Usage: uast [d|i|h|g]",
        ));
    }

    let f = match args.nth(1).unwrap_or_else(|| "d".to_string()).as_str() {
        "d" => uast::process_uast,
        "i" => iast::devanāgarī_to_iast,
        "g" => gu::devanāgarī_to_gujarātī,
        "-v" => {
            #[cfg(debug_assertions)]
            const BUILD_TYPE: &'static str = "debug";
            #[cfg(not(debug_assertions))]
            const BUILD_TYPE: &'static str = "release";

            println!(
                "{} {} ({BUILD_TYPE} build, {OS} [{ARCH}])",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            );
            println!("For web version, visit `https://uast.dev`");
            println!("For citations, visit `https://arxiv.org/abs/2203.14277`");

            return Ok(());
        }
        "-h" => {
            println!("Usage: uast [d|i|g]");

            return Ok(());
        }
        _ => {
            return Err(Error::new(ErrorKind::InvalidInput, "Usage: uast [d|i|g]"));
        }
    };

    loop {
        let mut l = String::new();

        match io::stdin().read_line(&mut l) {
            Err(e) => {
                return Err(e);
            }

            Ok(0) => {
                return Ok(());
            }

            _ => {
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
