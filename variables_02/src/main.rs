//! # CHAPTER 02 - Variables

use std::f32::consts::PI;

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut name = "Mic";
    let amount: i64 = 123456789123465;
    name = "Mr Mic";

    //INFO: Shadowing
    let color = "blue";
    let color = "red";
    println!("🪚 color: {:?}", color);

    let pi = PI;
    let million = 1_000_000;

    let is_day = true;
    println!("🪚, is_day: {:?}", is_day);

    let char = 'A';
    let char2 = '\u{1F601}';
    println!("🪚 char: {:?}, {}", char, char2);
}
