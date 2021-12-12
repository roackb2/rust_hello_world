mod modules;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    modules::primitives::play_around_primitives();
    modules::primitives::play_around_compound_types();
    modules::flow::test_statement();
    let c = modules::flow::simple_add(5, 8);
    println!("The result is {}.", c);
    modules::flow::control_flow();
    modules::ownership::ownership();
    modules::reference::reference();
    modules::slices::slice_type();

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
