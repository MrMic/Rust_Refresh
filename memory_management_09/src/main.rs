#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    // INFO: Lifetime elision!
    // fn get_name<'a>(&'a self) -> &'a String {
    fn get_name(&self) -> &String {
        &self.name
    }
}

// INFO: struct Object<'lifetime> {
// INFO:   field: &'lifetime Type,
// INFO:  ... }
#[derive(Debug)]
struct Dog<'a> {
    name: String,
    owner: &'a Person,
}

// INFO: Lifetime
fn main() {
    println!("{}", get_str());

    let p1 = Person {
        name: String::from("Alice"),
    };
    let d1 = Dog {
        name: String::from("Buddy"),
        owner: &p1,
    };
    println!("{:?}", d1);

    let a: &String;
    {
        let p2 = Person {
            name: String::from("Bob"),
        };
        // a = p2.get_name();
        a = p1.get_name();
    }
    println!("{}", a);
}

fn get_str() -> &'static str {
    "Hello!"
}
