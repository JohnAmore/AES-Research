mod reader;
// Reader
// Takes the data from "Sbox.txt" and returns the S-Box for main program usage.
// FILE I/O

use std::env;
use std::fs;

pub fn readFile() {
    let contents =
        fs::read_to_string("../sBox.txt").expect("Should have been able to read the file");

    println!("Content:\n{contents}");
}
