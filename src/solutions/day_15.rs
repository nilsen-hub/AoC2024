use std::{collections::VecDeque, time::Instant};

use crate::support::field_tools::{Field, Point};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> WareHouse {
        let mut field = Field::default();
        let mut directions = VecDeque::new();
        let mut position = Point::default();

        for line in self.input.lines() {
            if line.is_empty() {
                continue;
            }
            match &line[0..1] {
                "#" => field.field.push(line.chars().collect::<Vec<char>>()),
                "<" | ">" | "v" | "^" => directions = line.chars().collect::<VecDeque<char>>(),
                _ => continue,
            }
        }

        field.width = field.field[0].len() as isize;
        field.height = field.field.len() as isize;

        // find robot
        'outer: for (idy, line) in field.field.iter().enumerate() {
            for (idx, c) in line.iter().enumerate() {
                if *c == '@' {
                    position = Point::from((idx, idy));
                    break 'outer;
                }
            }
        }

        WareHouse {
            floor: field,
            robot: Robot {
                position,
                move_list: directions,
            },
        }
    }

    fn parse_part_2(&self) {}
}

#[derive(Debug, Clone, Default)]
struct WareHouse {
    floor: Field<char>,
    robot: Robot,
}

#[derive(Debug, Clone, Default)]
struct Robot {
    position: Point,
    move_list: VecDeque<char>,
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

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

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day fifteen answers:");
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
