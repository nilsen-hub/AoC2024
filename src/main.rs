use aoc2024::solutions;
use std::{env, io, process, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = vec![
        include_str!(".././data/full/day_1"),
        include_str!(".././data/full/day_2"),
        include_str!(".././data/full/day_3"),
        include_str!(".././data/full/day_4"),
        include_str!(".././data/full/day_5"),
        include_str!(".././data/full/day_6"),
        include_str!(".././data/full/day_7"),
        include_str!(".././data/full/day_8"),
        include_str!(".././data/full/day_9"),
        include_str!(".././data/full/day_10"),
        include_str!(".././data/full/day_11"),
        include_str!(".././data/full/day_12"),
        include_str!(".././data/full/day_13"),
        include_str!(".././data/full/day_14"),
    ];

    clear_terminal();
    print_header();

    match args.len() {
        1 => (),
        _ => run_command(&data, &args[1]),
    }

    print_intro(data.len());
    run_command(&data, &get_user_input());

    process::exit(0);
}

fn run_command(data: &Vec<&str>, command: &str) {
    clear_terminal();
    print_header();
    match command {
        "ALL" => run_all_days(data.clone()),
        _ => day_launcher(data.clone(), &command),
    }
    process::exit(0);
}

fn print_intro(len: usize) {
    println!("Which day would you like to run?");
    println!(
        "(Enter number from 1 to {}, or type \"ALL\" to run all days)",
        len
    );
    println!("");
}

fn clear_terminal() {
    if env::consts::OS == "windows" {
        std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input is missing");

    input.truncate(input.len() - 2);
    input
}

fn day_launcher(data: Vec<&str>, day: &str) {
    let day: usize = day.parse().unwrap();
    match day {
        1 => solutions::day_1::solution(data[day - 1]),
        2 => solutions::day_2::solution(data[day - 1]),
        3 => solutions::day_3::solution(data[day - 1]),
        4 => solutions::day_4::solution(data[day - 1]),
        5 => solutions::day_5::solution(data[day - 1]),
        6 => solutions::day_6::solution(data[day - 1]),
        7 => solutions::day_7::solution(data[day - 1]),
        8 => solutions::day_8::solution(data[day - 1]),
        9 => solutions::day_9::solution(data[day - 1]),
        10 => solutions::day_10::solution(data[day - 1]),
        11 => solutions::day_11::solution(data[day - 1]),
        12 => solutions::day_12::solution(data[day - 1]),
        13 => solutions::day_13::solution(data[day - 1]),
        14 => solutions::day_14::solution(data[day - 1]),
        _ => println!("{} is unavailable for some reason", day),
    }
    println!("");
}

fn run_all_days(data: Vec<&str>) {
    let to_send = data.clone();
    for (index, _day) in data.iter().enumerate() {
        day_launcher(to_send.clone(), &(index + 1).to_string());
        println!("");
    }

    process::exit(0);
}

fn print_header() {
    println!("");
    println!("-- Advent of code 2024 --");
    println!("");
}
