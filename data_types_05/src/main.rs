#![allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // INFO: Arrays are static & cannot be resized. They are allocated on the stack and have a
    // fixed size.
    let primes = [2, 3, 5, 7, 11];

    let doubles: [f64; 5] = [2.0, 3.0, 5.0, 7.0, 11.0];
    println!("➡ doubles: {:?}", doubles);

    let mut numbers = [0; 10];
    numbers[2] = 42;
    println!("➡️ numbers: {:?}", numbers);

    print!("󰅨 ");
    for number in numbers.iter() {
        print!("{}, ", number + 3)
    }
}
