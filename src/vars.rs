// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Barry";
    let age = 35;

    // age = 36; --> cannot mutate vars like this
    // proper syntax would be to assign orig var:
    // let mut age = 35;
    // age = 36;

    println!("My name is {0}, and I am {1} years old.", name, age);

    // Define constant --> must be uppercase and declare a type
    // in this case a 32-bit integer
    // https://doc.rust-lang.org/book/ch03-02-data-types.html
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars at once
    let ( my_name, my_age ) = ( "Barry", 35 );
    println!("{0} is {1}", my_name, my_age);

}