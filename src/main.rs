use clap::Parser;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

mod lexer;
mod location;
mod parser;
mod sexp;

use lexer::tokens;

fn run_str(input: &str) -> Result<(), io::Error> {
    let tok = tokens(input);
    for i in tok.iter() {
        println!("{:?}", i);
    }
    Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lock = stdout.lock();
    loop {
        write!(lock, "> ").unwrap();
        stdout.flush().unwrap();
        let mut buf = String::new();
        let n = stdin.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        run_str(&buf)?;
    }
    Ok(())
}

fn run_file(path: &PathBuf) -> Result<(), io::Error> {
    let content = fs::read_to_string(path)?;
    run_str(&content)
}

#[derive(Parser)]
struct Args {
    file: Option<PathBuf>,
}

fn main() -> Result<(), io::Error> {
    let cli = Args::parse();

    match cli.file {
        Some(p) => run_file(&p),
        None => run_prompt(),
    }
}

// fn make_token_number(&mut self, beg: Position) -> Token {
//     #[derive(PartialEq)]
//     enum CharType {
//         Dot,
//         Digit,
//         Other,
//     }

//     let mut was_dot = false;
//     loop {
//         let char_type = match self.peek() {
//             None => CharType::Other,
//             Some('.') => CharType::Dot,
//             Some(&ch) => {
//                 if ch.is_ascii_digit() {
//                     CharType::Digit
//                 } else {
//                     CharType::Other
//                 }
//             }
//         };
//         if was_dot && char_type != CharType::Digit {
//             return self
//                 .make_error(beg, "invalid number literal starting at position {beg.pos}");
//         } else if char_type == CharType::Digit {
//             was_dot = false;
//             self.next();
//         } else if char_type == CharType::Dot {
//             was_dot = true;
//             self.next();
//         } else {
//             return self.make_number(beg);
//         }
//     }
// }
