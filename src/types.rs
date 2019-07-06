/*
Primitive Types:
Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: (bool)
Characters: (char)
Tuples
Arrays
*/


// Rust is a statically typed language, which means that is must
// know the types of all variables at compile time, however, the compiler
// can usually infer the types based on the value and how we use it

pub fn run() {
    // Defualt is "i32"
    let x = 1;

    // Default is "f64"
    let y = 3.14;

    // Add explict type
    let z: i64 =31541361;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Character, emoji is also allowed, unicode
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}