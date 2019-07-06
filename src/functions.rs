// Function - used to store blocks of code for re-use

pub fn run() {
    greeting("hello", "Jane");

    // Bind function value to variables
    let get_sum = add(1, 2);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_sum = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure Sum: {}", add_sum(3, 3));
}

// Function without return value
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// Return value: without semicolon
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}