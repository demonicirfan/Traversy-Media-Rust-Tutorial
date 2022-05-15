// variables hold primitive data or references to data
// vaiables are immutable by default
// rust is a block scoped language

pub fn run(){
    let name = "Brad";

    let mut age = 37;

    age = 38;
    println!("my name is {}, age {}", name, age);

    //define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple variables at once
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}