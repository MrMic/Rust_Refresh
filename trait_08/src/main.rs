#![allow(unused_variables)]
#![allow(unused_assignments)]

//INFO: STRUCTS _________________________________________________________
struct Dog {}
struct Cat {}

//INFO: TRAITS & IMPL ___________________________________________________
trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "Meow!"
    }
}

// ______________________________________________________________________
fn get_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 1.0 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

// ______________________________________________________________________
fn main() {
    // WARN: Traits are similar to Interfaces in other languages, but they are not the same. They are
    // more powerful and flexible than interfaces in other languages.

    println!("The animal says: {}", get_animal(0.5).make_noise());
    println!("The animal says: {}", get_animal(1.5).make_noise());
}
