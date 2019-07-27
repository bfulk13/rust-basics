// Arrays - fixed list where elements are the same data types

// bring this in or you would need to use `std::mem::size_of_val(&numbers)` on line 21
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reassign value, but make the array `mut`
    numbers[3] = 13;

    println!("{:?}", numbers);

    // get single val
    println!("Single value: {}", numbers[0]);

    // get array length
    println!("Array Length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice from array
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}