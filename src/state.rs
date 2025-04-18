// State Box
// A "4x4" Vector of u8s.
// Takes 16 bytes(characters) out of the message and puts them in the State Box for encrypting.
//NOTE: Contents of State Box NEEDS to be mutable so that the AES algorithm can work.
use crate::message;
use crate::reader;
use crate::state;

pub struct State {
    pub state_box: Vec<u8>,
}

impl State {
    pub fn fill_state(&mut self, message: &mut message::Message) {
        for char in (0..16) {
            self.state_box[char] = message.content.as_bytes()[(message.at as usize)];
            message.at += 1;
            if message.at == message.content.len().try_into().unwrap() {
                for i in message.at as usize..self.state_box.len().try_into().unwrap() {
                    self.state_box[i] = 0;
                }
                break;
            }
        }
    }
}
