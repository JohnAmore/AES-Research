use reader::fill_s_box;

mod add_round_key;
mod keygen;
mod message;
mod mix_columns;
mod reader;
mod shift_rows;
mod state;
mod sub_bytes;
mod timer;
//AES
//
//

pub fn main() {
    //Fill s-box.
    let mut sub_box: sub_bytes::sub_box = sub_bytes::sub_box {
        sub_box: fill_s_box(),
    };
    //Key Generation

    //Get message & content.
    let mut data: message::Message = message::Message::create("./testData1.txt".to_string());
    //Initialize state.
    let mut state_box: state::State = state::State {
        state_box: vec![0u8; 16],
    };
    //Fill the state box with 16 bytes.
    state_box.fill_state(&mut data);
}
