// Substituion box
// Substitute each byte with the corresponding hex value.
// NOTE: Sub_Box needs to be mutable when instantiated.

use crate::state;
struct Sub_box {
    sub_box: Vec<u8>,
}
// Get the byte, return the substituted value
fn sub_char(val: u8, table: &Sub_box) -> u8 {
    let byte:u8  = val as u8;
    table.sub_box[byte as usize]
}
//NOTE: When calling this function, the state struct MUST be mutable.
fn sub_bytes(state: &mut state::State, sbox: &Sub_box) {
    for iter in (0..16) {
        state.state_box[iter] = sub_char(state.state_box[iter], sbox);
    }
}
