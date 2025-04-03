//AddRoundKey
//XOR with state_box
//NOTE: Document & implement.
use crate::keygen;
use crate::state;
pub fn add_round_key(state: &mut state::State, key: keygen::Key) {
    for i in 0..16 {
        state.state_box[i] = state.state_box[i] ^ key.key.as_bytes()[key.at as usize];
    }
}

pub fn add_round_key_improved(state: &mut state::State, key1: keygen::Key, key2: keygen::Key) {
    for i in 0..16{
        let idx = i as usize;
        if idx % 2 == 0 {
            //TODO: Finish
        } else {
            state.state_box[idx] = state.state_box[idx] ^ key1.key.as_bytes()[key1.at as usize];
        }
    }
}
