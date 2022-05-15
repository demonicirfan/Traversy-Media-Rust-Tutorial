// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("hello");

    //get length
    println!("Length: {}", hello.len());
    hello.push(' ');
    hello.push('w');
    hello.push_str("orld");
    println!("{}", hello);

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // is empty
    println!("Is Empty: {}", hello.is_empty());

    //contains
    println!("Contains 'world': {}", hello.contains("world"));

    println!("REPLACE: {}", hello.replace("world", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    //assertion testing
    assert_eq!(3, s.len());
    assert_eq!(1, s.capacity());
}
