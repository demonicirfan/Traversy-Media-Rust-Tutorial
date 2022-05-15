/**
 * Primitive types -
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
 * Floats: f32, f64
 * Booleans: bool
 * Characters: char
 * Tuples
 * Arrays
 */

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    //by default it will be i32
    let x = 1;
    // f64
    let y = 2.5;

    //add explicit type
    let z: i64 = 65656565;
    // find  max size
    println!("Mx i32: {}", std::i32::MAX);
    println!("Mx i64: {}", std::i64::MAX);

    //bolean
    let is_active: bool = 10 < 5;
    println!("{:?}", (x, y, z, is_active));

    let a1='\u{1f600}';
    println!("{}", a1);
}
