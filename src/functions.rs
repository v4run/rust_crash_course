pub fn run() {
    println!("functions.rs");
    println!("{}", big(10, 2));
    print_big(12, 3);

    // Closures
    let c: i32 = 3;
    let adder = |a: i32, b: i32| a + b + c;
    println!("Sum closed over 'c': {}", adder(1, 2));
    block_value();
}

fn print_big(a: i32, b: i32) {
    if a > b {
        println!("{}", a);
    } else {
        println!("{}", b);
    }
}

fn big(a: i32, b: i32) -> i32 {
    if a > b {
        a // skip ; to return the value
    } else {
        b
    }
}

fn block_value() {
    println!("\nBlock value demo.");
    let a = if 10 > 2 {
        println!("10");
        10
    } else {
        println!("2");
        2
    };
    let value = {
        let mut i = 0;
        loop {
            i += 1;
            if i == 20 {
                break;
            }
        }
        i
    };
    println!("a = {}", a);
    println!("value = {}", value);
}
