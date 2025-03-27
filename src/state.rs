// State Box
// A "4x4" Vector of u8s.
// Takes 16 bytes out of the message and puts them in the State Box for encrypting.
//NOTE: Contents of State Box NEEDS to be mutable so that the AES algorithm can work.
mod reader;
struct State {
    state_box: Vec<u8>,
    message: String,
    filename: String,
}

impl State {
    //TODO: Complete function
    fn fill_state() {}
}
