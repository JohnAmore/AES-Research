// Substituion box
// Substitute each byte with the corresponding hex value.
// NOTE: Sub_Box needs to be mutable when instantiated.
mod state;
struct Sub_box {
    sub_box: Vec<u8>,
}
// Get the byte, return the substituted value
fn sub_char(val: String, table: Sub_box) -> char {
    let byte: u8 = val as u8;
    Sub_box.sub_box[byte]
}
//NOTE: When calling this function, the state struct MUST be mutable.
fn subBytes(state: &mut State, sbox: Sub_box) {
    for iter in (0..16) {
        state.state_box[iter] = sub_char(state.state_box[iter], sbox);
    }
}
