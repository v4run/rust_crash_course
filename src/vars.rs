// variables hold primitive data or references to data
// variables are immutable by default
// Rust is block scoped

pub fn run() {
    let name = "Varun"; // name is immutable
    let mut age = 27; // age is mutable
    println!("My name is {} and I am {}", name, age);
    age = 28;
    println!("My name is {} and I am {}", name, age);

    // define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    {
        // assign multiple variables
        let (name, age) = ("Varun", 27);
        println!("My name is {}. I am {}", name, age); // age has block scope.
    }

    println!("My name is {}. I am {}", name, age);
}
