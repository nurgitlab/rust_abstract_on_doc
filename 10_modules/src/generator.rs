use rand::Rng;

mod random_number;
// use random_number::RandomNumber;

use random_number::*;
// use crate::generator::random_number::RandomNumber;

pub fn generate () -> u8 {
    let random_num = rand::rng().random_range(0..=10);
    return RandomNumber::new(random_num as u8).value;
}