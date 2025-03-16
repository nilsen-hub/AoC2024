use aoc2024::{solutions, support::aoc_qol::clear_terminal};
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
        include_str!(".././data/full/day_15"),
        include_str!(".././data/full/day_16"),
        include_str!(".././data/full/day_17"),
        include_str!(".././data/full/day_18"),
        include_str!(".././data/full/day_19"),
    ];
    let test_data = vec![
        include_str!(".././data/test/day_1"),
        include_str!(".././data/test/day_2"),
        include_str!(".././data/test/day_3"),
        include_str!(".././data/test/day_4"),
        include_str!(".././data/test/day_5"),
        include_str!(".././data/test/day_6"),
        include_str!(".././data/test/day_7"),
        include_str!(".././data/test/day_8"),
        include_str!(".././data/test/day_9"),
        include_str!(".././data/test/day_10"),
        include_str!(".././data/test/day_11"),
        include_str!(".././data/test/day_12"),
        include_str!(".././data/test/day_13"),
        include_str!(".././data/test/day_14"),
        include_str!(".././data/test/day_15"),
        include_str!(".././data/test/day_16"),
        include_str!(".././data/test/day_17"),
        include_str!(".././data/test/day_18"),
        include_str!(".././data/test/day_19"),
    ];

    clear_terminal();
    print_header();

    match args.len() {
        1 => (),
        _ => run_command(&data, &args[1], &test_data),
    }

    print_intro(data.len());
    run_command(&data, &get_user_input(), &test_data);

    process::exit(0);
}

fn run_command(data: &Vec<&str>, command: &str, test_data: &Vec<&str>) {
    clear_terminal();
    print_header();
    match command {
        "ALL" => run_all_days(data.clone(), test_data.clone()),
        _ => day_launcher(data.clone(), &command, test_data.clone()),
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

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input is missing");

    input.truncate(input.len() - 2);
    input
}

fn day_launcher(data: Vec<&str>, day: &str, test_data: Vec<&str>) {
    let day: usize = day.parse().unwrap();
    match day {
        1 => solutions::day_1::solution(data[day - 1], test_data[day - 1]),
        2 => solutions::day_2::solution(data[day - 1], test_data[day - 1]),
        3 => solutions::day_3::solution(data[day - 1], test_data[day - 1]),
        4 => solutions::day_4::solution(data[day - 1], test_data[day - 1]),
        5 => solutions::day_5::solution(data[day - 1], test_data[day - 1]),
        6 => solutions::day_6::solution(data[day - 1], test_data[day - 1]),
        7 => solutions::day_7::solution(data[day - 1], test_data[day - 1]),
        8 => solutions::day_8::solution(data[day - 1], test_data[day - 1]),
        9 => solutions::day_9::solution(data[day - 1], test_data[day - 1]),
        10 => solutions::day_10::solution(data[day - 1], test_data[day - 1]),
        11 => solutions::day_11::solution(data[day - 1], test_data[day - 1]),
        12 => solutions::day_12::solution(data[day - 1], test_data[day - 1]),
        13 => solutions::day_13::solution(data[day - 1], test_data[day - 1]),
        14 => solutions::day_14::solution(data[day - 1], test_data[day - 1]),
        15 => solutions::day_15::solution(data[day - 1], test_data[day - 1]),
        16 => solutions::day_16::solution(data[day - 1], test_data[day - 1]),
        17 => solutions::day_17::solution(data[day - 1], test_data[day - 1]),
        18 => solutions::day_18::solution(data[day - 1], test_data[day - 1]),
        19 => solutions::day_19::solution(data[day - 1], test_data[day - 1]),
        _ => println!("{} is unavailable for some reason", day),
    }
    println!("");
}

fn run_all_days(data: Vec<&str>, test_data: Vec<&str>) {
    let now = Instant::now();
    let to_send = data.clone();
    let test = test_data.clone();
    for (index, _day) in data.iter().enumerate() {
        day_launcher(to_send.clone(), &(index + 1).to_string(), test.clone());
        println!("");
    }
    println!("Total runtime: {}", now.elapsed().as_secs_f32());
    process::exit(0);
}

fn print_header() {
    println!("");
    println!("-- Advent of code 2024 --");
    println!("");
}
