//! Driver-code

use std::{
    env::{
        self,
        consts::{ARCH, OS},
    },
    io::{self, Error, ErrorKind},
};
use uast::{devanāgarī_to_gujarātī, devanāgarī_to_iast, process_uast};

fn main() -> Result<(), Error> {
    let mut args = env::args();
    if args.len() > 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments. Usage: uast [d|i|h|g]",
        ));
    }

    let f = match args.nth(1).unwrap_or_else(|| "d".to_string()).as_str() {
        "d" => process_uast,
        "i" => devanāgarī_to_iast,
        "g" => devanāgarī_to_gujarātī,
        "-v" => {
            #[cfg(debug_assertions)]
            const BUILD_TYPE: &str = "debug";
            #[cfg(not(debug_assertions))]
            const BUILD_TYPE: &str = "release";

            println!(
                "{} {} ({BUILD_TYPE} build, {OS} [{ARCH}])",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            );
            println!("For web version, visit `https://uast.dev`");
            println!(
                "For citations, visit Unicode Aware Saṃskṛta Transliteration [Dalwadi and Dave, 2022: `https://arxiv.org/html/2203.14277`]"
            );

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
                println!("{}", f(&l));
            }
        };
    }
}
