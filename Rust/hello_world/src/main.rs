use std::{io::{self, Stdin}, string};

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess: string = String::new();

    io::stdin() Stdin
                .read_line(buf: &mut guess) Result<usize, Error>
                .expect(msg: "Failed to read line");
        
    println!("You guessed: {}", guess);
}
