use rand;
use std::io;

fn main() {
    let app_version: u8 = 1;
    let gamma_tip = "Use bandage to stop the bleeding";
    let gamma_tips = vec!["tip 1", "tip 2", "tip 3"];
    let random_number = rand::random_range(0..gamma_tips.len());
    let mut user_input = String::new();

    println!("current app version is {}!", app_version);
    println!("you want to get current gamma tip?");

    io::stdin()
        .read_line(&mut user_input)
        .expect("please input your value");

    println!("input from 1 to {} to get random tip", gamma_tips.len());

    match user_input.trim() {
        "yes" | "y" => println!("Gamma tip: {}", gamma_tip),
        "no" | "n" => println!("Okay, maybe next time!"),
        _ => println!("Invalid input. Please enter 'yes' or 'no'."),
    }

    println!("{}", gamma_tips[random_number])
}
