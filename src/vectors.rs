// vectors - resizable arrays

pub fn vectors() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // add to vector
    numbers.push(6);

    // get single value
    println!("Single Value: {}", numbers[0]);

    for elem in numbers.iter_mut() {
        *elem *= 2;
    }

    println!("{:?}", numbers);

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
