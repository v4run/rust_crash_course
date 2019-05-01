/*
 * Primitive Types
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
 * Floats: f32, f64
 * Boolean: bool
 * Character: char
 * Tuples
 * Arrays
 */

// Rust is a statically typed language with type inference

pub fn run() {
    let x = 1; // default is "i32"
    let y = 2.5; // "f64"

    // let y: i64 = 123456789; // explicit type. okay to redefine 'y' using let
    let z: i64 = 123456789;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);

    // boolean
    let ok = true;

    let (a, b) = (1, 2);
    let is_greater = a > b;

    let a1 = 'a'; // char. a unicode character
    let face = '\u{1f600}'; // 😀

    println!("{:?}", (ok, x, y, z, is_greater, a1, face));
}
