//AddRoundKey
//XOR with state_box
//NOTE: Document & implement.
use crate::keygen;
use crate::state;
use std::fs::File;
use std::io::Write;
pub fn add_round_key(state: &mut state::State, key: keygen::Key) {
    for i in 0..16 {
        state.state_box[i] = state.state_box[i] ^ key.content.as_bytes()[key.at as usize];
    }
}

pub fn add_round_key_improved(state: &mut state::State, key: keygen::Key) {
    for i in 0..16 {
        let idx = i as usize;
        let key1 = &key.content[0..key.content.len() / 2];
        let key2 = &key.content[(key.content.len() / 2 + 1)..key.content.len()];

        if idx % 2 == 0 {
            state.state_box[idx] = state.state_box[idx] ^ key2.as_bytes()[key.at as usize];
        } else {
            state.state_box[idx] = state.state_box[idx] ^ key1.as_bytes()[key.at as usize];
        }
    }
}

pub fn export_key(key: keygen::Key) -> std::io::Result<()> {
    //Exports key to a file
    let mut file = File::create("key.txt")?; // Create or overwrite a file
    file.write_all(key.content.as_bytes())?; // Write data as bytes
    Ok(())
}

pub fn xor_key_improved_aes(key: keygen::Key) {
    let key1 = &key.content[0..key.content.len() / 2];
    let key2 = &key.content[(key.content.len() / 2 + 1)..key.content.len()];
}
