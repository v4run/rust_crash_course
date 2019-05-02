enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

pub fn run() {
    move_it(Movement::Up);
    move_it(Movement::Down);
    move_it(Movement::Left);
    move_it(Movement::Right);
}

fn move_it(m: Movement) {
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right"),
    }
}
