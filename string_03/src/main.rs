#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // INFO: String Slices - ⚠️ => IMMUTABLE
    // let cat = "Fluffy";
    let cat: &'static str = "Fluffy";
    println!("🪚 {:?}", cat);

    // INFO: String Objects - ⚠️ => IMMUTABLE
    let dog = String::new();
    let mut dog = String::from("Max");
    println!("🪚 {:?}", dog);

    // INFO: format! Macro
    let owner = format!("{} is the owner of {}", "Alice", cat);
    println!("🪚 {:?}", owner);

    println!("🪚 Length of cat: {}", cat.len());
    dog.push(' ');
    dog.push_str("the dog");
    println!("🪚 {:?}", dog);
    let new_dog = dog.replace("Max", "Buddy");
    println!("🪚 {:?}", new_dog);
}
