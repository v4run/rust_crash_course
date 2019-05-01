// Tuples are a group of values of different types
// Maximum of 12 elements

pub fn run() {
    let person: (&str, i8) = ("Varun", 27);
    println!("My name is {}. I am {}.", person.0, person.1);
}
