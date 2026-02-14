#![allow(dead_code)]

#[allow(unused_variables)]
fn main() {
    // NOTE: STRUCTURE
    let emp = Employe {
        name: String::from("John"),
        company: String::from("Google"),
        age: 35,
    };

    println!("emp => {:?}", emp);
    println!("emp.name => {}", emp.name);
    println!("emp.company => {}", emp.company);
    println!("emp.age => {}", emp.age);

    println!("-------------------------");

    println!("  emp => {}", emp.fn_details());
    // println!("emp => {}", Employe::fn_details(&emp));

    println!("{}", Employe::static_fn_details());
}

#[derive(Debug)]
struct Employe {
    name: String,
    company: String,
    age: u32,
}

// WARN: Add method to the struct
impl Employe {
    fn fn_details(&self) -> String {
        format!(
            "name: {}, company: {}, age: {}",
            &self.name, &self.company, &self.age
        )
    }

    // WARN: Static method ( without &self)
    fn static_fn_details() -> String {
        String::from("👨‍🍼  Details of an employe")
    }
}
