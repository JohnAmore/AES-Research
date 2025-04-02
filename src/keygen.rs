// Key generation
// Use RNG to produce an alphanumeric key
use rand::prelude::*;
struct Key{
    key: String,
}
pub fn gen_key_AES() -> String{
    let mut key = String::new();
    for char in (0..16){
        key.push(rng.sample(rand::distr::Alphanumeric) as char););
    }
    key
}

//NOTE: For tweaked use after normal AES
pub fn gen_key_tweaked_AES() -> String {
let mut key = String::new();
    for char in (0..32){
        key.push(rng.sample(rand::distr::Alphanumeric) as char););
    }
    key
}
