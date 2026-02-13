#![allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // INFO: Tuples: Collection of values of varous types
    // WARN: Static, cannot be resized - Elements can be updated
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("🪚 person => {:?}", person);
    println!("🪚 person.0 => {:?}", person.0);
    println!("------------------------");

    person.2 = false;
    println!("🪚 person => {:?}", person);
    println!("------------------------");

    let (name, age, employed) = person;
    println!("🪚 name: {}, age: {}, employed: {}", name, age, employed); // WARN: Destructuring
}
