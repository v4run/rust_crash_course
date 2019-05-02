pub fn run() {
    let age = 21;

    if age >= 23 {
        println!("You can drink");
    } else {
        println!("You shall not DRINK!!!!!");
    }

    let is_of_age = if age >= 23 { true } else { false };
    println!("Is of age? {}", is_of_age);
}
