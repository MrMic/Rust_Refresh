#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let i = 32;
    let j = i;
    println!("i: {}, j: {}", i, j);

    let v = vec![1, 2, 3];
    // let w = v;
    // println!("w: {:?}", w);
    // WARN: The following line will cause a compile-time error because v has been moved to w
    // println!("v: {:?}", v);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("vector used in foo: {:?}", v);
        v
    };

    let v = foo(v);
    println!("v after foo: {:?}", v);
}
