// conditionals - used to check the condition of something and act accordingly

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("what you want to drink");
    } else if age < 21 && check_id {
        println!("sorry you have to go home");
    } else {
        println!(" i will need to see your id");
    }

    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("is_of_age: {}", is_of_age);
}
