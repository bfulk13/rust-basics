// Conditionals - use to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = false;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, I cannot serve you.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    // shorthand if (Rust's shorthand if is similar to a ternary operator in ES6+)
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age? {}", is_of_age);
}