use crate::support::field_tools as field;
use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

fn part_1(data: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let input = data.input.lines().collect::<Vec<&str>>();
    let field = parse_input(input);

    for (index, line) in field.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            if *c == 'X' {
                acc += find_xmas(&field, index, idx)
            }
        }
    }

    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(data: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let input = data.input.lines().collect::<Vec<&str>>();

    let field = parse_input(input);
    for (index, line) in field.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            if *c == 'M' {
                let mas_amount = find_masx(&field, index, idx);
                if mas_amount > 0 {
                    acc += mas_amount;
                }
            }
        }
    }

    println!("Part two: {}", acc / 2);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn find_masx(field: &Vec<Vec<char>>, index: usize, idx: usize) -> usize {
    let mut acc: usize = 0;
    let north: Vec<(usize, usize)> = vec![(index - 1, idx), (index - 2, idx), (index - 3, idx)];
    let north_west: Vec<(usize, usize)> = vec![(index - 1, idx + 1), (index - 2, idx + 2)];
    let west: Vec<(usize, usize)> = vec![(index, idx + 1), (index, idx + 2)];
    let south_west: Vec<(usize, usize)> = vec![(index + 1, idx + 1), (index + 2, idx + 2)];
    let south: Vec<(usize, usize)> = vec![(index + 1, idx), (index + 2, idx)];
    let south_east: Vec<(usize, usize)> = vec![(index + 1, idx - 1), (index + 2, idx - 2)];
    let east: Vec<(usize, usize)> = vec![(index, idx - 1), (index, idx - 2)];
    let north_east: Vec<(usize, usize)> = vec![(index - 1, idx - 1), (index - 2, idx - 2)];
    let x_directions: Vec<Vec<(usize, usize)>> =
        vec![north_west, south_west, south_east, north_east];
    let field_north = field[north[1].0][north[1].1];
    let field_south = field[south[1].0][south[1].1];
    let field_west = field[west[1].0][west[1].1];
    let field_east = field[east[1].0][east[1].1];
    for (num, direction) in x_directions.iter().enumerate() {
        let mut mas: String = String::new();
        for index in direction {
            mas.push(field[index.0][index.1]);
        }
        if mas == "AS" {
            match num {
                0 => {
                    if field_north == 'M' && field_west == 'S'
                        || field_north == 'S' && field_west == 'M'
                    {
                        acc += 1;
                    }
                }
                1 => {
                    if field_south == 'M' && field_west == 'S'
                        || field_south == 'S' && field_west == 'M'
                    {
                        acc += 1;
                    }
                }
                2 => {
                    if field_south == 'M' && field_east == 'S'
                        || field_south == 'S' && field_east == 'M'
                    {
                        acc += 1;
                    }
                }
                3 => {
                    if field_north == 'M' && field_east == 'S'
                        || field_north == 'S' && field_east == 'M'
                    {
                        acc += 1;
                    }
                }
                _ => println!("you should never ever see this.."),
            };
        }
    }

    acc
}

fn find_xmas(field: &Vec<Vec<char>>, index: usize, idx: usize) -> usize {
    let mut output_acc: usize = 0;
    let north: Vec<(usize, usize)> = vec![(index - 1, idx), (index - 2, idx), (index - 3, idx)];
    let north_west: Vec<(usize, usize)> = vec![
        (index - 1, idx + 1),
        (index - 2, idx + 2),
        (index - 3, idx + 3),
    ];
    let west: Vec<(usize, usize)> = vec![(index, idx + 1), (index, idx + 2), (index, idx + 3)];
    let south_west: Vec<(usize, usize)> = vec![
        (index + 1, idx + 1),
        (index + 2, idx + 2),
        (index + 3, idx + 3),
    ];
    let south: Vec<(usize, usize)> = vec![(index + 1, idx), (index + 2, idx), (index + 3, idx)];
    let south_east: Vec<(usize, usize)> = vec![
        (index + 1, idx - 1),
        (index + 2, idx - 2),
        (index + 3, idx - 3),
    ];
    let east: Vec<(usize, usize)> = vec![(index, idx - 1), (index, idx - 2), (index, idx - 3)];
    let north_east: Vec<(usize, usize)> = vec![
        (index - 1, idx - 1),
        (index - 2, idx - 2),
        (index - 3, idx - 3),
    ];
    let directions: Vec<Vec<(usize, usize)>> = vec![
        north, north_west, west, south_west, south, south_east, east, north_east,
    ];
    for direction in directions {
        let mut mas: String = String::new();
        for index in direction {
            mas.push(field[index.0][index.1]);
        }
        if mas == "MAS" {
            output_acc += 1;
        }
    }

    output_acc
}

fn parse_input(full_data: Vec<&str>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::with_capacity(146);
    let width = full_data[1].len() + 6;
    let mut count = 3;
    while count > 0 {
        let to_output = vec!['0'; width];
        output.push(to_output);
        count -= 1;
    }

    for line in full_data {
        let to_output = format!("000{}000", line);
        output.push(to_output.chars().collect());
    }
    count = 3;
    while count > 0 {
        let to_output: Vec<char> = vec!['0'; width];
        output.push(to_output);
        count -= 1;
    }
    output
}

pub fn solution(path: &str) {
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };

    println!("Running day four");
    println!("");

    println!("Answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_analysis() {}
}
