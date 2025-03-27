mod reader;
pub fn main() {
    let s_box = reader::fill_s_box();
    for hex in s_box {
        println!("{}", hex);
    }
}
