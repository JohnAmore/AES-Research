// Reader
// Takes the data from "Sbox.txt" and returns the S-Box for main program usage.
// FILE I/O
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
//NOTE: Future use function.
//pub fn read_message(filename: String) -> String {
//    read_to_string(filename).expect("Should have been able to read the file")
//}

pub fn fill_s_box() -> Vec<String> {
    let hexes: Vec<String> = read_lines("./sBox.txt");
    let mut s_box = Vec::new();
    for hex in hexes {
        s_box.push(hex);
    }
    s_box
}
