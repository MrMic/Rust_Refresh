#![allow(dead_code)]
use crate::Colors::Blue;
use crate::Person::*;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

// INFO: Add Data Type to enum's members
#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
}

#[allow(unused_variables)]
fn main() {
    let my_color = Colors::Red;
    println!("My color is: {my_color:?}");

    let my_color = Blue;
    println!("------------------------");

    let person = Name("John".to_string());
    println!("Person is: {person:?}");
}
