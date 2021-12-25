mod playground;
mod binary_tree;
mod merkle_tree;
mod linked_list;
mod json;

use std::io;
use crate::playground::{ play, guess::guessing_game };
use crate::json::test_json;
use crate::merkle_tree::test_tree;
use crate::linked_list::test_list;

fn main() {
    test_tree();
    test_json();
    play();
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
