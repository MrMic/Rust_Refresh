#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fs::File;

fn main() {
    // let f = File::open("main.jpeg").unwrap();
    let f = File::open("main.jpeg").expect("‼️ Failed to open main.jpeg");
}
