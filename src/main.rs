#[macro_use] extern crate itertools;
extern crate futures;
extern crate levenshtein;

mod util;
mod solutions;

use std::env;
use solutions::*;

fn solve(day: i32, question: i32) {
    match day {
        1 => day01::solve(question),
        2 => day02::solve(question),
        _ => println!("Unknown day {:?}", day)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("Missing day number").parse::<i32>().expect("Invalid day number");
    let question = args.get(2).expect("Missing question index").parse::<i32>().expect("Invalid question index");

    println!("Solving question #{:?} for day {:?}", question, day);

    solve(day, question);
}
