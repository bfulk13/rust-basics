// Loops - used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // infinite loop, break set at 20
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count === 20 {
    //         break;
    //     }
    // }

    // while loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else if count % 3 == 0 {
    //         println!("Fizz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     // increment
    //     count += 1;
    // }

    // For Range
    for x in 0..101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", x);
        }
    }
}