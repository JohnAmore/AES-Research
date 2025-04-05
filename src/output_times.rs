use std::fs::OpenOptions;
use std::io::Write;

pub fn output_times_aes(time: f32) {
    let path = "AESoutput.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    writeln!(file, "{}", time).expect("Failed to write to file");
}
pub fn output_times_aes_tweak(time: f32) {
    let path = "AESTweakedoutput.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open file");

    writeln!(file, "{}", time).expect("Failed to write to file");
}
