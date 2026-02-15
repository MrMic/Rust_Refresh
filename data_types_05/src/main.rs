#![allow(dead_code)]
use Colors::*;

#[allow(unused_variables)]
fn main() {
    let p1: Point<i32> = Point { x: 5, y: 10 };
    let p2: Point<f32> = Point { x: 1.0, y: 4.0 };
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let c1 = Red("#ff0000");
    println!("c1: {:?}", c1);
}

// ______________________________________________________________________
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Blue(T),
    Green(T),
}
