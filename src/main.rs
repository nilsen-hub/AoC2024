use aoc2024::solutions;
use std::{env, fs, io, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let days = fs::read_dir("./data/full/").unwrap();
    let day_count = days.into_iter().count();

    print_header();

    if args.len() > 1 {
        if args[1] == "ALL" {
            run_all(day_count);
        }
        day_launcher(&args[1]);
        process::exit(0);
    }

    println!("Which day would you like to run?");
    println!(
        "(Enter number from 1 to {}, or type \"ALL\" to run all days)",
        day_count
    );
    println!("");

    let mut instruction = String::new();

    io::stdin()
        .read_line(&mut instruction)
        .expect("Input is missing");

    let len = instruction.len();
    instruction.truncate(len - 2);

    print!("{esc}c", esc = 27 as char);

    print_header();
    if instruction == "ALL" {
        run_all(day_count);
    }
    day_launcher(&instruction);
    //process::exit(0);
}

fn day_launcher(day: &str) {
    match day {
        "1" => solutions::day_1::solution("./data/full/day_1"),
        "2" => solutions::day_2::solution("./data/full/day_2"),
        "3" => solutions::day_3::solution("./data/full/day_3"),
        "4" => solutions::day_4::solution("./data/full/day_4"),
        "5" => solutions::day_5::solution("./data/full/day_5"),
        "6" => solutions::day_6::solution("./data/full/day_6"),
        "7" => solutions::day_7::solution("./data/full/day_7"),
        _ => println!("{} is unavliable for some reason", day),
    }
    println!("");
}

fn run_all(day_count: usize) {
    for num in 1..day_count + 1 {
        day_launcher(num.to_string().as_str());
        println!("");
    }

    process::exit(0);
}

fn print_header() {
    println!("");
    println!("-- Advent of code 2024 --");
    println!("");
}
