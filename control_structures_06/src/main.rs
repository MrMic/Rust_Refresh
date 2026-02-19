#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    for i in 0..15 {
        println!("Amount: {}, I have : {}", i, get_oranges(i));
    }
}

// INFO: match statements can also have guards, which are additional conditions that must be met
// for a pattern to match. Guards are specified using the `if` keyword followed by a boolean
// expression. In the example below we use a guard to check if the amount of oranges is even when
// it exceeds 10.
fn get_oranges(amount: i32) -> &'static str {
    match amount {
        0 => "No oranges",
        1..=4 => "A few oranges",
        5..=10 => "Several oranges",
        _ if (amount % 2 == 0) => "an even amount of oranges",
        _ => "Many oranges",
    }
}
