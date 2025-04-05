// Reader
// Takes the data from "Sbox.txt" and returns the S-Box for main program usage.
// FILE I/O
use std::fs::read_to_string;

fn read_lines(filename: String) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
pub fn read_message(filename: String) -> String {
    read_to_string(filename).expect("Should have been able to read the file")
}

pub fn fill_s_box() -> Vec<u8> {
    let hexes: Vec<String> = read_lines("./sBox.txt".to_string());
    let mut s_box = Vec::new();

    for hex in hexes {
        let hex = hex.trim();
        if let Ok(byte) = u8::from_str_radix(hex, 16) {
            s_box.push(byte);
        } else {
            eprintln!("Invalid hex string: {}", hex);
        }
    }
    assert!(s_box.len() == 256, "The s-box is not big enough");

    s_box
}
