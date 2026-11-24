//! Driver-code

use std::{
    env::{
        self,
        consts::{ARCH, OS},
    },
    io::{self, BufRead, Error, ErrorKind, Write},
};
use uast::*;

const USAGE: &str = "Usage: uast [d|i|g|s]";

fn print_version(out: &mut impl Write) -> io::Result<()> {
    #[cfg(debug_assertions)]
    const BUILD_TYPE: &str = "debug";
    #[cfg(not(debug_assertions))]
    const BUILD_TYPE: &str = "release";

    writeln!(
        out,
        "{} {} ({BUILD_TYPE} build, {OS} [{ARCH}])",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )?;
    writeln!(out, "For web version, visit `https://uast.dev`")?;
    writeln!(
        out,
        "For citations, visit Unicode Aware Saṃskṛta Transliteration [`https://arxiv.org/html/2203.14277`]"
    )
}

/// Treats a closed stdout (for example `uast | head -n 1`) as a normal exit instead of an error.
fn ignore_broken_pipe(result: io::Result<()>) -> io::Result<()> {
    match result {
        Err(e) if e.kind() == ErrorKind::BrokenPipe => Ok(()),
        other => other,
    }
}

fn main() -> io::Result<()> {
    let mut args = env::args_os();
    if args.len() > 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid number of arguments. {USAGE}"),
        ));
    }

    let mode = match args.nth(1) {
        None => "d".to_string(),
        Some(arg) => arg
            .into_string()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, USAGE))?,
    };

    let mut stdout = io::stdout().lock();

    let f: fn(&str) -> String = match mode.as_str() {
        "d" => uast_to_devanāgarī,
        "i" => devanāgarī_to_iast,
        "g" => devanāgarī_to_gujarātī,
        "s" => slp_to_iast,

        "-v" | "--version" => return ignore_broken_pipe(print_version(&mut stdout)),
        "-h" | "--help" => return ignore_broken_pipe(writeln!(stdout, "{USAGE}")),
        _ => {
            return Err(Error::new(ErrorKind::InvalidInput, USAGE));
        }
    };

    let mut stdin = io::stdin().lock();
    let mut line = Vec::new();

    loop {
        line.clear();

        // Read raw bytes and decode lossily: invalid UTF-8 becomes U+FFFD, which every converter
        // drops, instead of aborting the rest of the input.
        if stdin.read_until(b'\n', &mut line)? == 0 {
            return Ok(());
        }

        let converted = f(&String::from_utf8_lossy(&line));
        if let Err(e) = writeln!(stdout, "{converted}") {
            return ignore_broken_pipe(Err(e));
        }
    }
}
