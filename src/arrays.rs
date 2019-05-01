// arrays are fixed length list of elements of same data type
use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // numbers[0] = 10; // not allowed for immutable arrays
    println!("{:?}", numbers);
    // array elements
    println!("{}", numbers[0]);

    let mut mutable_numbers: [i32; 5] = [1, 2, 3, 4, 5];
    mutable_numbers[0] = 10;
    println!("{:?}", mutable_numbers);

    // length of array
    println!("Array length: {}", numbers.len());

    // find size used by the array. arrays are allocated in stack
    println!("Size: {} bytes", mem::size_of_val(&numbers));

    // slices
    let slice_full: &[i32] = &numbers;
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice_full);
    println!("Slice 0..2 {:?}", slice);
}
