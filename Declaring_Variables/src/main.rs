/*
Credit:
https://www.rustfinity.com/practice/rust/challenges/declaring-variables
Write a program that prints "Hello, World!" 
to the console using the println! macro.
*/
pub fn calculate_area() -> u32 {
    // TODO: Implement the function here
    // 1. Declare a variable named width
    // 2. Declare a variable named height
    // 3. Run the `prints_values` function with the width and height variables
    // 4. Return the multiplication of width and height
    let width: u32 = 30;
    let height:u32 = 30;
    prints_values(width, height);
    width * height
}

// WARNING: Do not modify this function
pub fn prints_values(width: u32, height: u32) {
    println!("The width is: {}", width);
    println!("The height is: {}", height);
}