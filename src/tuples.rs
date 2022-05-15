// Tuples group togther values of differetn types
// max 12 elements
use std::mem;

pub fn tuples() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    numbers[2] = 20;
    for elem in numbers {
        println!("{}", elem);
    }
    println!("{}", numbers.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
