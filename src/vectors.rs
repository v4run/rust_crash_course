// vectors are resizable arrays
use std::mem;

pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // numbers[0] = 10; // not allowed for immutable vectors
    // numbers.push(10); // not allowed for immutable vectors
    println!("{:?}", numbers);
    // vector elements
    println!("{}", numbers[0]);

    let mut mutable_numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    mutable_numbers[0] = 10;
    println!("Original {:?}", mutable_numbers);
    mutable_numbers.push(10);
    println!("After push {:?}", mutable_numbers);
    mutable_numbers.pop();
    println!("After pop {:?}", mutable_numbers);

    // length of vector
    println!("Array length: {}", numbers.len());

    // find size used by the vector. vectors are allocated in stack
    println!("Size: {} bytes", mem::size_of_val(&numbers));

    // slices
    let slice_full: &[i32] = &numbers;
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice_full);
    println!("Slice 0..2 {:?}", slice);

    // iterating over vector
    for x in numbers.iter() {
        println!("{}", x);
    }

    println!("Original {:?}", mutable_numbers);

    // iterating over vector and mutating the values
    for x in mutable_numbers.iter_mut() {
        *x += 10;
    }
    println!("After adding 10 {:?}", mutable_numbers);
}
