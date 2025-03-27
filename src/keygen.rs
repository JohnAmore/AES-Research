// Key generation
// Use RNG to produce an alphanumeric key
use rand::prelude::*;

pub fn gen_key() -> String{
    let mut key = String::new();
    for char in (0..256){
        key.push(rng.sample(rand::distr::Alphanumeric) as char););
    }
    key
}
