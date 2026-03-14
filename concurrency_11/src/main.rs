#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{sync::mpsc, thread};

fn main() {
    /*
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    println!("Received: {}", rx.recv().unwrap());
    */

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });
    for received in rx {
        println!("Received: {}", received);
    }
}
