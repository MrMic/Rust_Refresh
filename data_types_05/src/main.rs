#![allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // SLICE:
    // WARN: A Slice is a pointer to a block of memory
    // Can be used on arrays, vectors and strings
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("slice => {:?}", slice);

    // ______________________________________________________________________
    let mut colors = ["red", "green", "blue", "pink"];
    println!("colors => {:?}", colors);

    update_colors(&mut colors[2..4]);
    println!("colors => {:?}", colors)
}

fn update_colors(color_slice: &mut [&str]) {
    color_slice[0] = "yellow";
    color_slice[1] = "orange";
}
