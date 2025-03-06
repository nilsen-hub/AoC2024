use aoc2024::solutions;
use std::{env, fs, io, process};
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
    ];

    print_header();

    //if args.len() > 1 {
    //    //if args[1] == "ALL" {
    //    //    run_all(day_count);
    //    //}
    //    day_launcher(&args[1], day_1);
    //    process::exit(0);
    //}

    println!("Which day would you like to run?");
    println!(
        "(Enter number from 1 to {}, or type \"ALL\" to run all days)",
        data.len()
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
    //if instruction == "ALL" {
    //    run_all(day_count);
    //}
    day_launcher(data, &instruction);
    //process::exit(0);
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
        _ => println!("{} is unavliable for some reason", day),
    }
    println!("");
}

fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    // Get a list of all entries in the folder
    let entries = fs::read_dir(path)?;

    // Extract the filenames from the directory entries and store them in a vector
    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}

//fn run_all(day_count: usize) {
//    for num in 1..day_count + 1 {
//        day_launcher(num.to_string().as_str());
//        println!("");
//    }
//
//    process::exit(0);
//}

fn print_header() {
    println!("");
    println!("-- Advent of code 2024 --");
    println!("");
}
