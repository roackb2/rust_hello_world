mod modules;
mod structures;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use crate::modules::primitives::{
    play_around_primitives,
    play_around_compound_types
};
use crate::modules::flow::{
    test_statement,
    simple_add,
    control_flow
};
use crate::modules::ownership::ownership;
use crate::modules::reference::reference;
use crate::modules::slices::slice_type;
use crate::modules::structs::test_structs;
use crate::modules::enums::test_enums;
use crate::structures::merkle_tree::test_tree;

fn main() {
    play_around_primitives();
    play_around_compound_types();
    test_statement();
    let c = simple_add(5, 8);
    println!("The result is {}.", c);
    control_flow();
    ownership();
    reference();
    slice_type();
    test_structs();
    test_enums();
    test_tree();

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
