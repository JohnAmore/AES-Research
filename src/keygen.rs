// Key generation
// Use RNG to produce an alphanumeric key
use rand::distributions::Alphanumeric;
use rand::prelude::*;

pub struct Key {
    key: String,
}

pub fn gen_key_AES() -> String {
    let mut key = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..16 {
        key.push(rng.sample(Alphanumeric) as char);
    }
    key
}

// NOTE: For tweaked use after normal AES
pub fn gen_key_tweaked_AES() -> String {
    let mut key = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..32 {
        key.push(rng.sample(Alphanumeric) as char);
    }
    key
}
