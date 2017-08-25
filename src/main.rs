#![feature(ascii_ctype)]

#[macro_use]
extern crate lazy_static;

mod lexer;

use std::io::{self, Read};

fn main() {
    let mut lexer = lexer::Lexer::new("stdin");
    
    for c in io::stdin().bytes().map(|b| b.unwrap()) {
        match lexer.push_char(c) {
            Ok(Some(out)) => {
                print!("{} ", out);
            }
            Ok(_) => {}
            Err(e) => {
                println!("\nError: {}", e);
                return;
            }
        }
    }

    println!();
}
