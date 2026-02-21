#![allow(unused_variables)]
#![allow(unused_assignments)]

//INFO: STRUCTS _________________________________________________________
struct RustDev {
    awesome: bool,
}

struct JSDev {
    awesome: bool,
}

//INFO: TRAITS & IMPL ___________________________________________________
trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello world!")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("Hello from Rust!")
    }
}

impl Developer for JSDev {
    fn new(awesome: bool) -> Self {
        JSDev { awesome }
    }

    fn language(&self) -> &str {
        "JavaScript"
    }

    fn say_hello(&self) {
        println!("Hello from JavaScript!")
    }
}

// ______________________________________________________________________
fn main() {
    // WARN: Traits are similar to Interfaces in other languages, but they are not the same. They are
    // more powerful and flexible than interfaces in other languages.

    let r1 = RustDev { awesome: true };
    let r2 = RustDev::new(true);
    println!("{} is awesome: {}", r1.language(), r1.awesome);
    println!("{} is awesome: {}", r2.language(), r1.awesome);
    r1.say_hello();
    r2.say_hello();
    println!("-----------------------------");

    let js1 = JSDev { awesome: false };
    let js2 = JSDev::new(false);
    println!("{} is awesome: {}", js1.language(), js1.awesome);
    println!("{} is awesome: {}", js2.language(), js2.awesome);
    js1.say_hello();
    js2.say_hello();
    println!("-----------------------------");
}
