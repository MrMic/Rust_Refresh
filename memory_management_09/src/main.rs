#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::rc::Rc;

struct Car {
    brand: Rc<String>,
}

impl Car {
    fn new(brand: Rc<String>) -> Self {
        Car { brand }
    }
    fn drive(&self) {
        println!("Driving a {} car!", self.brand);
    }
}

// INFO: Reference Counter
// - Rc<T> (Reference Counted Smart Pointer)
fn main() {
    let brand = Rc::new(String::from("Toyota"));
    println!("Reference count: {}", Rc::strong_count(&brand));
    {
        let car2 = Car::new(brand.clone());
        println!("Reference count: {}", Rc::strong_count(&brand));
        car2.drive();
    }
    println!("Reference count: {}", Rc::strong_count(&brand));
}
