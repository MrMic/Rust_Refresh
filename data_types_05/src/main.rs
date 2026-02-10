#![allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // INFO: Vector
    let primes: Vec<i32> = Vec::new();

    let mut primes = vec![2, 3, 5];
    println!("{:?} ", primes);

    primes.push(7);
    println!("{:?} ", primes);

    primes.remove(2);
    println!("{:?} ", primes);

    let mut numbers = vec![2; 6];
    println!("{:?} ", numbers);
    numbers[5] = 8;
    println!("{:?} ", numbers);
    for n in numbers.iter() {
        print!("{:?}, ", n);
    }
}
