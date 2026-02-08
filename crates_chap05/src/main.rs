// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
use rand::{Rng, distr::Alphanumeric};

mod archive;

fn main() {
    // arch_file("example.txt");
    arc("example.txt");

    let mut rng = rand::rng();
    let a: i32 = rng.random();
    println!("Random number: {}", a);

    println!("Bounded int: {}", rng.random_range(1..=10));
    println!("Bounded float: {}", rng.random_range(0.0..=100.0));

    let rand_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(10) // INFO: Only 10 chars
        .map(char::from)
        .collect();
    println!("Random string: {}", rand_string);
}
