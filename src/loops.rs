pub fn run() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Number: {}", count);
        if count == 20 {
            break;
        }
    }

    count = 0;

    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    for x in 0..100 {
        println!("{}", x);
    }
}
