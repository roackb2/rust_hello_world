use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    play_around_numbers();

    let mut input = String::new();

    println!("Should we play the guessing game? \
        type 'play' to play. \
    ");

    io::stdin()
        .read_line(&mut input)
        .expect("Fail to parse input");

    match input.trim() {
        "play" => guessing_game(),
        _ => println!("Alright, don't play")
    };
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        println!("Please input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        println!("You input the guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Nanah, numbers only.");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}

fn play_around_numbers() {
    let mut some_num = 5;
    println!("Some number: {}", some_num);
    some_num = 6;
    println!("Some number is changed to {}", some_num);
    let some_num: u32 = 32;
    println!("Shadowing some number to {}", some_num);
    // unsigned numbers could not be negative
    // let some_num = -64;
    let some_num = 23_987;
    println!("Decimal representation of 23_98: {}", some_num);
    let some_num = 0xab;
    println!("Hex representation of 0xab: {}", some_num);
    let some_num = 0o32;
    println!("Octal representation of 0o32: {}", some_num);
    let some_num = 0b1001_0100;
    println!("Binary representation of 0b1001_0100: {}", some_num);
}