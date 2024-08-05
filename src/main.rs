//! Driver-code

use std::io;

mod uast;

fn main() {
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

                let x = l
                    .trim()
                    .split_whitespace()
                    .map(|x| uast::process_uast(x.to_string()))
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{x}");
            }
        };
    }
}
