use crate::support::field_tools::Field;
use crate::support::field_tools::Point;
use std::collections::VecDeque;
use std::str::FromStr;
use std::{fs::read_to_string, time::Instant};

// TTB pt. 1: 450 micros, pt 2: 76 ms
#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> Lab {
        let floorplan = Field::from_str(&self.input).unwrap();

        for (idy, line) in floorplan.field.iter().enumerate() {
            for (idx, char) in line.iter().enumerate() {
                if *char == '^' {
                    let directions = vec![Dir::North, Dir::East, Dir::South, Dir::West];
                    let guard = Guard {
                        position: Point::from(idx as isize, idy as isize),
                        direction: VecDeque::from(directions),
                    };
                    return Lab { floorplan, guard };
                }
            }
        }

        return Lab::default();
    }
    fn parse_part_2(&self) {}
}

#[derive(Debug, Clone, Default)]
enum Dir {
    #[default]
    North,
    East,
    South,
    West,
}
#[derive(Debug, Clone, Default)]
struct Lab {
    floorplan: Field,
    guard: Guard,
}

impl Lab {
    fn solve_part_one(&self) -> usize {
        let dummy = 0;

        dummy
    }
}

#[derive(Debug, Clone, Default)]
struct Guard {
    position: Point,
    direction: VecDeque<Dir>,
}

fn part_1(input: &InputData) {
    let mut acc = 0;
    let now = Instant::now();
    let parsed = input.parse_part_1();

    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let parsed = input.parse_part_2();

    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(path: &str) {
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };

    println!("Running day six");
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
