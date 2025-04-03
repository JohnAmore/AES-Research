// State Box
// A "4x4" Vector of u8s.
// Takes 16 bytes(characters) out of the message and puts them in the State Box for encrypting.
//NOTE: Contents of State Box NEEDS to be mutable so that the AES algorithm can work.
//TODO: Refactor, use message struct that removes message & filename fields from here.
use crate::state;
use crate::reader;
use crate::message;

pub struct State {
    pub state_box: Vec<u8>,
}

impl State {
    //TODO: Complete function
    fn fill_state(&mut self,message: &mut message::Message) {
        for char in (0..16) {
            self.state_box[char] = message.content.as_bytes()[(message.at as usize + 1) + char];
        }
    }
}
