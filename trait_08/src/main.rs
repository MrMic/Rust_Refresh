#![allow(unused_variables)]
#![allow(unused_assignments)]

//INFO: STRUCTS _________________________________________________________

//INFO: TRAITS & IMPL ___________________________________________________
trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for &num in self {
            sum += num;
        }
        sum
    }
}

// ______________________________________________________________________
fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let result = a.sum();
    println!("The sum of the vector is: {}", result);
}
