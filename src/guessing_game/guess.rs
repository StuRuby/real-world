use std::io;

pub fn guess_num() {
    println!("Guess the number");
    println!("Please enter the number you guessed");

    let mut guess_str = String::new();
    io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed to read line");

    println!("Your guess number is {}", guess_str);
}