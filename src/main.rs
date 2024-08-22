//! Driver-code

use std::{env, io};

mod iast;
mod uast;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("Invalid number of arguments. Usage: uast d|i");
    }

    let devanāgarī_mode = match args.nth(1).unwrap().as_str() {
        "d" => true,
        "i" => false,
        _ => panic!("Invalid argument. Usage: uast d|i"),
    };

    loop {
        let mut l = String::new();

        match io::stdin().read_line(&mut l) {
            Err(e) => {
                panic!("{e}");
            }

            Ok(e) => {
                if e == 0 {
                    return;
                }

                if devanāgarī_mode {
                    let x = l
                        .split_whitespace()
                        .map(|x| uast::process_uast(x.to_string()))
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{x}");
                } else {
                    let x = l
                        .split_whitespace()
                        .map(|x| iast::devanāgarī_to_iast(x.to_string()))
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{x}");
                }
            }
        };
    }
}
