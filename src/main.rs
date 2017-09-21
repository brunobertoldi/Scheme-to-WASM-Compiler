#![feature(log_syntax)]
#![feature(ascii_ctype)]

#[macro_use]
extern crate lazy_static;

mod iter;
mod lexer;

use std::io::{self, Read};

fn main() {
    let lexer = lexer::Lexer::new("stdin");

    let source = io::stdin().bytes().map(|b| b.unwrap()).chain(vec![b'\n'].into_iter());
    for res in lexer.iter(source) {
        match res {
            Ok(t) => print!("{} ", t),
            Err(e) => {
                println!("\nError: {}", e);
                return;
            }
        }
    }

    println!();
}
