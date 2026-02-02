/*
Credit:
https://www.rustfinity.com/practice/rust/challenges/constants
Define a constant named MAX_SIZE with a value of 100.
Ensure that MAX_SIZE has the type i32.
Return the value of MAX_SIZE from the main function.
Make sure you make the constant public using the pub keyword (important for the tests to pass).
Remember that constants can only be defined at the global scope. You can not define them inside a function.
*/

// Define a constant MAX_SIZE with a value of 100.
// NOTE: Define the constant outside the main function
// Your code here

pub const MAX_SIZE: i32 = 100; 

pub fn main() -> i32 {
    MAX_SIZE
}

