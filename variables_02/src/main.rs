//! # CHAPTER 02 - Variables

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut name = "Mic";
    let amount: i64 = 123456789123465;
    name = "Mr Mic";

    //INFO: Shadowing
    let color = "blue";
    let color = "red";
    println!("🪚 color: {:?}", color);
}
