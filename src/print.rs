pub fn run() {
    // print to console
    println!("Hello from print.rs!");

    // basic formatting
    println!("{} is a {}", "Barry", "BEAST!!!");

    // positional arguments
    println!("{0} is from {1}, and {0} likes to {2}.", "Barry", "Paris", "code");

    // named arguments
    println!("{name} likes to play {activity}.", name = "Barry", activity = "basketball");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait ** will use often **
    // this is also called a tuple
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}