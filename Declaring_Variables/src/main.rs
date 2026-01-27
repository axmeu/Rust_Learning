/*
Credit:
https://www.rustfinity.com/practice/rust/challenges/declaring-variables
The calculate_area function should:
Declare variables for width and height.
Use the prints_values function to display the values of the width and height.
Return the calculated area of the rectangle by multiplying width and height.
Do not modify the prints_values function.
*/
fn main() {
    let result = calculate_area();
}

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