pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("Number: {}", 10);
    println!("{} sets in the {}", "Sun", "West");

    // positional arguments
    println!("{0} is {1}. Also {0} is {2}.", 1, "one", "ONE");

    // named arguments
    println!(
        "{number_one} is {one}. Also {number_one} is {ONE}.",
        number_one = 1,
        one = "one",
        ONE = "ONE"
    );

    // placeholder traits
    println!("Decimal: {0} Binary: {0:b} Hex: {0:x} Octal: {0:o}", 10);

    // placeholder for debug trait
    println!("{:?}", ("varun", 27, true));

    // basic math
    println!("1 + 2 = {}", 1 + 2);
}
