#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::ops::Add;

//INFO: STRUCTS _________________________________________________________
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//INFO: TRAITS & IMPL ___________________________________________________

// ______________________________________________________________________
fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}
