/*
 *  Primitive Types:
 *      -Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (# of bits in memory)
 *          * `u` designates no negative values
 *      -Floats: f32, f64
 *      -Boolean (bool)
 *      -Characters (char)
 *      -Tuples
 *      -Arrays
*/

// Rust is a statically typed language. It must know the types of all
// variables at compile time. However, the compiler can usually infer what
// what type we want to use based on the value and how we use it.

pub fn run() {
    // default is i32
    let x = 1;

    // default is f64
    let y = 2.5;

    // add explicit type
    let z: i64 = 23463456352345646;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get a Boolean from an expression
    let is_greater = 10 > 5;

    // declare a char with single quote
    let a1 = 'a';
    let face = '\u{1F600}';


    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}