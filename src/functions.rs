pub fn run() {
    greeting("hello", "Brad");
    println!("{}", add(4, 6));

    // bind funciton values to variables
    let get_sum = add(8, 6);

    //closure
    let n3: i32 = 10;
    // Rust closures are anonymous functions without any name that you can save in a variable or pass as arguments to other functions. Unlike functions, closures can capture values from the scope in which they’re defined.
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("c_Sum {}", add_nums(3, 3))
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
