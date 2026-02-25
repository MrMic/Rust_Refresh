#![allow(unused_variables)]
#![allow(unused_assignments)]

//INFO: STRUCTS _________________________________________________________

//INFO: TRAITS & IMPL ___________________________________________________
trait Duplicateable {
    fn duplicate(&self) -> String;
}

// ______________________________________________________________________
impl Duplicateable for String {
    fn duplicate(&self) -> String {
        format!("{} - {}", self, self)
    }
}

impl Duplicateable for i32 {
    fn duplicate(&self) -> String {
        format!("{}", self * 2)
    }
}

// ______________________________________________________________________
fn duplicate<T: Duplicateable>(x: T) {
    println!("{}", x.duplicate());
}

// ______________________________________________________________________
fn main() {
    let a = 12;
    let b = String::from("Hello");

    duplicate(a);
    duplicate(b);
}
