#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // INFO: High Order Functions
    let square = |a: i32| a * a;
    apply(square, 5);
}

// ______________________________________________________________________
fn apply<F>(f: F, a: i32)
where
    F: Fn(i32) -> i32,
{
    println!("Result: {}", f(a));
}
