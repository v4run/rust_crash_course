// Pointers: points to a reference in memory

pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // with non-primitives, if you assign another variable to a piece of data,
    // the first variables will no longer hold that value. So we will have to
    // use a reference(&) to point to that data.

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vectors: {:?}", (&vec1, vec2));
}
