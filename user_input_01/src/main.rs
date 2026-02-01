fn main() {
    println!("Hello, my name is: {}", "Mic");
    println!("{0} has {1} dog.", "Toto", 2 + 1);
    println!("{name} has a {pet}.", name = "Alice", pet = "cat");
    println!("binary: {:b} - Hex: {:x} - Octal: {:o}", 5, 5, 5);
    println!("Array: {:?}", [1, 2, 3]);
    println!("Right aligned: {:>5}", 10);
}
