#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{
    fs::{File, remove_file},
    io::Read,
};

fn main() {
    // INFO: Create a file
    // let mut file = File::create("file_handling_10/src/example.txt").expect("Failed to create file");

    // INFO: Write to a file
    // file.write_all(b"Hello, world!").
    //     .expect("Failed to write to file");la_eti

    // INFO: Append to a file
    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .open("file_handling_10/src/example.txt")
    //     .expect("Failed to open file");
    // file.write_all(b"\nAppending a new line to the file.")
    //     .expect("Failed to write to file");

    // INFO: Read from a file
    let mut file = File::open("file_handling_10/src/example.txt").unwrap_or_else(|err| {
        eprintln!("Failed to open file: {}", err);
        std::process::exit(1);
    });
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        std::process::exit(1);
    });
    println!("File content:\n{}", content);

    // INFO: Remove a file
    remove_file("file_handling_10/src/example.txt").unwrap_or_else(|err| {
        eprintln!("Failed to remove file: {}", err);
        std::process::exit(1);
    });
}
