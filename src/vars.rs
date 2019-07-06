// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Kurt";
    let mut age = 21;
    println!("My name is {} and I am {}", name, age);
    age = 22;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);


    // Assign mutliple vars
    let (my_name, my_age) = ("Bard", 32);
    println!("{} is {}", my_name, my_age);
}