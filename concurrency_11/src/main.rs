use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let mut threads = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || {
            sleep(Duration::from_millis(i * 100));
            println!("Thread {:?}", i);
        });
        threads.push(th);
    }

    for th in threads {
        th.join().unwrap();
    }
    println!("Main thread");
}
