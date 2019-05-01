// Primitive str: immutable fixed-length string in stack
// String: growable, heap-allocated data structure

pub fn run() {
    let name = "Varun"; // "str" type
    let mut hello = String::from("Hello ");

    // length of String
    println!("{}", hello.len());

    // length of str
    println!("{}", name.len());

    // pushing char
    hello.push('W');

    // pushing string
    hello.push_str("orld!");

    // capacity of the string
    println!("Capacity: {}", hello.capacity());

    // check if string is empty
    println!("Empty? {}", hello.is_empty());

    // check for substring
    println!("Contains 'World'? {}", hello.contains("World"));

    // replace
    println!("After replacing. {}", hello.replace("World", "There"));

    // splitting string
    let words = hello.split_whitespace();
    println!("Split by space: {:?}", words);
    for w in words {
        println!("{}", w);
    }

    // create a string with defined capacity
    let mut s = String::with_capacity(10);
    s.push('N');
    s.push('O');
    println!("{}", s);

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (&hello, name)); // & is required to prevent multiple owners
    println!("{}", hello);
}
