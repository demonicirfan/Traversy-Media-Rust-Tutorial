pub fn run(){
    //print to console
    println!("Hello from the print.rs file");
    println!("Number {}", 1);
    println!("{} is from {}", "Brad", "Mars");
    println!("{name} likes to play {activity}", name="Brad", activity="Baseball");
    println!("binary: {:b}, hex: {:x} octal: {:0}", 10, 10, 10);
    println!("{:?}", (12, true, "hello"));
    println!("10+10 = {}", 10+10);
}