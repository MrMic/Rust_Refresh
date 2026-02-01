//! Crate about reading line from IO
use std::io;

/// Main function
/// ```
///fn main()
/// ```
/// Reads a line from stdin and echoes it back to the user.
fn main() {
    let mut input = String::new();

    // Prompt the user for input
    // fn main()
    println!("Say something:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said: {}", input.trim());
        }
        Err(e) => {
            eprintln!("Something went wrong: {}", e);
        }
    }
}
