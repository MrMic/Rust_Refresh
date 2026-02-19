#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fmt::Display;

fn main() {
    // INFO: Closures are anonymous functions that can capture variables from their surrounding
    // scope. They are often used for short-lived operations or when you want to pass a function as
    // an argument to another function.
    let a = |a: i32| -> i32 { a + 1 };
    println!("a(5) = {}", a(5));

    // INFO: Generic functions are functions that can operate on different types without needing to
    fn prn<T: Display + ?Sized>(x: &T) {
        println!("{}", x)
    }
    prn("Hello, world!");
    prn(&42);
}
