// structs - Used to create custom data types

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
// tuple struct
struct Color2(u8, u8, u8);

//struct with functions

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    //construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    let b = Color2(255, 0, 0);

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    println!("Color: {} {} {}", b.0, b.1, b.2);

    let mut p = Person::new("Mary", "Doe");
    p.set_last_name("william");
    println!("Person - {}", p.full_name())
}
