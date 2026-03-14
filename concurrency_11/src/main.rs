#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

fn main() {
    let lock = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10 {
        let lock_clone = Arc::clone(&lock);
        let thread = spawn(move || {
            let mut num = lock_clone.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented the number to {}", i, *num);
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Final number: {}", *lock.lock().unwrap());
}
