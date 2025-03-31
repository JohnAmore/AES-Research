// Key generation
// Use RNG to produce an alphanumeric key
use rand::prelude::*;

pub fn gen_key_AES() -> String{
    let mut key = String::new();
    for char in (0..128){
        key.push(rng.sample(rand::distr::Alphanumeric) as char););
    }
    key
}

//NOTE: For tweaked use after normal AES
pub fn gen_key_tweaked_AES() -> String {
let mut key = String::new();
    for char in (0..256){
        key.push(rng.sample(rand::distr::Alphanumeric) as char););
    }
    key
}
