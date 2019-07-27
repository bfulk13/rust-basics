// Vectors - resizeable arrays (likely to use more than arrays)

// bring this in or you would need to use `std::mem::size_of_val(&numbers)` on line 21
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // reassign value, but make the vector `mut`
    numbers[3] = 13;

    // add onto vector
    numbers.push(6);
    numbers.push(7);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get single val
    println!("Single value: {}", numbers[0]);

    // get vector length
    println!("Vector Length: {}", numbers.len());

    // arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice from array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // loop through numbers
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    println!("Numbers before Vec: {:?}", numbers);

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    // to print the whole vector use `{:?}`
    println!("Numbers after Vec: {:?}", numbers);
}