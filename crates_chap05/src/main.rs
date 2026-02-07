// use crate::archive::arch::arch_file;
use crate::archive::arch::arch_file as arc;
use rand::Rng;

mod archive;

fn main() {
    // arch_file("example.txt");
    arc("example.txt");

    let mut rng = rand::rng();
    let a: i32 = rng.random();
    println!("Random number: {}", a);
}
