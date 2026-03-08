#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fs::File;

fn main() {
    // INFO: Recoverable Error with: Result enum
    // INFO: enum Result<T, E> {
    // INFO:     Ok(T),
    // INFO:     Err(E),
    //   }

    let f = File::open("main.jpeg");
    match f {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
            // You can work with the file here
        }
        Err(e) => {
            println!("Failed to open the file: {}", e);
            // Handle the error, e.g., by creating a new file or exiting
        }
    }
    println!("Continuing with the rest of the program...");

    // INFO: Recoverable Error with: Option enum
    // INFO: enum Option<T,E> {
    // INFO:     Some(T),
    // INFO:     None,
    // INFO: }

    divide(Some(2));
    divide(None);
    divide(Some(0));
}

// ______________________________________________________________________
const ANSWER_TO_LIFE: i32 = 42;

fn divide(x: Option<i32>) {
    match x {
        Some(0) => panic!("⚠️ Cannot divide by zero!"),
        Some(x) => println!("✅ result is: {}", ANSWER_TO_LIFE / x),
        None => println!("‼️ No number provided!"),
    }
}
