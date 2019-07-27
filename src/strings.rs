// Primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run() {
    // primitive
    // let hello = "Hello";

    // growable
    let mut hello = String::from("Hello ");

    // get length
    println!("Length: {}", hello.len());

    // push a char (must be `mut` and growable with `String::from("")` syntax)
    hello.push('W');

    // push a string (must be `mut` and growable with `String::from("")` syntax)
    hello.push_str("orld!");

    // capacity method in bytes
    println!("Capacity: {}", hello.capacity());

    // is_empty method returns bool
    println!("Is Empty: {}", hello.is_empty());

    // contains method returns bool
    println!("Contains 'World' {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word); // prints L1 = Hello, L2 = World!
    }

    // create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing only prints when !=
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());

    println!("{}", s);

    // println!("{}", hello);
}