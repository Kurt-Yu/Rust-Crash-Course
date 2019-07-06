// reference pointers - Pointer to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // With non-primitives, if you assign another variavble to a piece of adata, 
    // the first variable will no longer hold that value
    // You'll need to use a reference (&) to point to that resource

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}