#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{fs::File, io::Error, io::Read};

// fn read_username_from_file() -> Result<String, Error> {
//     let f = File::open("file_handling_10/src/username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("file_handling_10/src/username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let a = read_username_from_file();
    println!("{:?}", a);
}
