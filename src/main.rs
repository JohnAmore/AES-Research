// In-crate modules.
use add_round_key::add_round_key;
use reader::fill_s_box;
mod add_round_key;
mod keygen;
mod message;
mod mix_columns;
mod output_times;
mod reader;
mod shift_rows;
mod state;
mod sub_bytes;
mod timer;
//AES
// Get S-box set up, generate key, grab message, fill state, start time
//

pub fn main() {
    //No time recorded here. Record after state filled.

    let mut times: Vec<f32> = Vec::new();
    //Fill s-box.
    let sub_box: sub_bytes::sub_box = sub_bytes::sub_box {
        sub_box: fill_s_box(),
    };
    //Key Generation
    let key: keygen::Key = keygen::Key {
        content: keygen::gen_key_content_aes(),
        at: 0,
    };
    //Get message & content.
    let mut data: message::Message = message::Message::create("./testData1.txt".to_string());
    //Initialize state.
    let mut state_box: state::State = state::State {
        state_box: vec![0u8; 16],
    };
    //Start time
    let time = timer::get_time();

    //Iterations/rounds
    while data.at < data.content.len().try_into().unwrap() {
        //Fill the state box with 16 bytes.
        state_box.fill_state(&mut data);
        for _ in 0..16 {
            sub_bytes::sub_bytes(&mut state_box, &sub_box);
            shift_rows::shift_rows(&mut state_box);
            mix_columns::mix_columns(&mut state_box);
            add_round_key(&mut state_box, key.clone());
        }
        println!("{:?}", state_box.state_box);
    }
    let time_taken = timer::get_time_taken(time);

    times.push(time_taken);

    for time in times {
        output_times::output_times_aes(time);
    }
}
