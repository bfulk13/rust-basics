// Structs - used to create custom data types similar to Classes in other languages

// Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

/***********Tuple Struct************/
// struct Color (u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name (&self operates similar to `this` keyword)
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // c.0 = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    /***********Tuple Struct************/
    // let mut c = Color(255, 0, 0);
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    /**************************/
    let mut p = Person::new("Jane", "Doe");
    println!("Person: {}", p.full_name());
    p.set_last_name("Wilson");
    println!("Person: {}", p.full_name());
    println!("Person to tuple: {:?}", p.to_tuple());
}