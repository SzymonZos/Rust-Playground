extern crate rand;

use rand::Rng;

pub fn generate_rng() {
    let mut rng = rand::thread_rng();
    let b:i32 = rng.gen();
    println!("Rng = {}", b);
}