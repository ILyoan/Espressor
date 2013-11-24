#[link(name = "espressor",
       vers = "0.1-pre")];

#[feature(globs)];

extern mod grinder;

use std::io;

use grinder::lexer::Lexer;

fn main() {
    let src = ~"var a=1; var b=2; a+b;";
    let mut lexer = Lexer::new(src);
    loop {
        match lexer.next_token() {
            Some(token) => io::println(format!("{:?}", token)),
            None => break
        }
    }
}

