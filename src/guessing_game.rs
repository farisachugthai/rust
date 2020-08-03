#!/usr/bin/env rustc
// Dude holy shit this shebang works correctly!
// If you run `./src/guessing_game.rs` this will compile and create
// an executable!
//
// The first part of this guessing game will ask a user for input,
// process that input and check that the input is in an expected form

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {}", guess);
}

fn _explanation() {
    let _foo = 5;  // immutable

    let mut _bar = 5;  // mutable

    let _string = String::new();  // a function that returns a growable, UTF-8 encoded bit of text

    /* The :: syntax in teh ::new line indicates that new is
     * an associated function of the String type. This function
     * is implemented on a type, in this fcase String rather than
     * than on the particular String instance.
     * This is referred to in a language such as Python as a static method.
     * Also the reason everything is prefaced with an underscore
     * is to avoid the compiler complaining about unused variables.
     */
}

