#!/usr/bin/env rust
/*
`println!` is a **macro** that prints to the console. A **macro** is like a function that writes
code for you. Macros have a `!` after them. We will learn about making macros later. For now,
remember that `!` means that it is a macro.
*/

fn main() {
    println!("Hello, world number {}!", 8);

    // All chars are 4 bytes. They are 4 bytes because some characters in a string are more than one byte. For example:
    let slice = "Hello!";
    println!("Slice is {:?} bytes.", std::mem::size_of_val(slice)); // std::mem::size_of_val gives the size in bytes
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {:?} bytes.", std::mem::size_of_val(slice2));
}
