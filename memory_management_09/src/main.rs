#![allow(unused_variables)]
#![allow(unused_assignments)]

// INFO: Borrowing and References
fn main() {
    let mut a = 6;
    {
        let b = &mut a;
        println!("b: {}", *b);
        *b += 2;
    }
    println!("a: {}", a);

    //---------------------------
    println!("---------------------------");
    let v = vec![1, 2, 3];
    for i in &v {
        println!("i: {}", i);

        // WARN: cannot borrow `v` as mutable because it is also borrowed as immutable
        // v.push(4);
    }
}
