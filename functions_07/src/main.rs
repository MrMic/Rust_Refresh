#![allow(unused_variables)]
#![allow(unused_assignments)]

// ______________________________________________________________________
// INFO:
// DESIGNATOR: 1. MACROS
// expr, ident, block, stmt, pat, path, meta, ty, tt ...
macro_rules! my_macro {
    () => {
        println!("This is a macro!");
    };
}

macro_rules! name {
    ($name: expr) => {
        println!("Hello, {}!", $name);
    };
}

macro_rules! name2 {
    ($($name: expr),*) => {
        $(println!("Hello, {}!", $name);)*
    };
}

macro_rules! build_fn {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("{:?} was called", stringify!($fn_name));
        }
    };
}

// ______________________________________________________________________
fn main() {
    // INFO: MACROS
    // macro_rules! my_macro {
    //  (match) => ( code to run )
    // }
    my_macro!();
    println!("--------------------");
    name!("Alice");
    println!("--------------------");
    name2!("Alice", "Bob", "Charlie");
    println!("--------------------");
    build_fn!(foo);
    foo();
}
