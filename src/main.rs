#![feature(log_syntax)]
#![feature(ascii_ctype)]

#[macro_use]
extern crate lazy_static;

mod iter;
mod lexer;

use std::io::{self, Read};

fn main() {
    let mut lexer = lexer::Lexer::new("stdin");

    let mut tokens = Vec::new();
    for c in io::stdin().bytes().map(|b| b.unwrap()).chain(vec![b'\n'].into_iter()) {
        match lexer.push_char(c) {
            Ok(Some(out)) => tokens.push(out),
            Ok(_) => {}
            Err(e) => {
                println!("\nError: {}", e);
                return;
            }
        }
    }

    for t in &tokens {
        print!("{}", t);
    }
    println!();
}
