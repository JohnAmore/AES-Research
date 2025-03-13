use std::fs;

pub fn main() {
    let contents =
        fs::read_to_string("./sBox.txt").expect("Should have been able to read the file");

    println!("Content:\n{contents}");
}
