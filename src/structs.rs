pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut ct = ColorT(255, 0, 0);
    ct.0 = 200;
    println!("Color: {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("Tony", "Stark");
    println!("Person full name: {}", p.full_name());
    p.set_last_name("Howard Stark");
    println!("Person full name: {}", p.full_name());

    println!("Person as tuple: {:?}", p.to_tuple());
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct ColorT(u8, u8, u8);
