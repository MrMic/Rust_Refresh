#![allow(unused_variables)]
#![allow(unused_assignments)]

//INFO: STRUCTS _________________________________________________________
struct Dog {
    species: &'static str,
}

struct Cat {
    color: &'static str,
}

//INFO: TRAITS & IMPL ___________________________________________________
trait Bark {
    fn bark(&self) -> String;
}

impl Bark for Dog {
    fn bark(&self) -> String {
        format!("{} says: Woof!", self.species)
    }
}

// ______________________________________________________________________
fn bark_it<T: Bark>(animal: T) {
    println!("{}", animal.bark());
}

// ______________________________________________________________________
fn main() {
    // WARN: Traits are similar to Interfaces in other languages, but they are not the same. They are
    // more powerful and flexible than interfaces in other languages.

    let dog = Dog { species: "Bulldog" };
    let cat = Cat { color: "Black" };

    bark_it(dog);
    // bark_it(cat); // WARN: Cat does not implement the Bark trait
}
