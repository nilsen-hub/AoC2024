use crate::support::field_tools::{Field, Point};
use std::{collections::VecDeque, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> WareHouse {
        let mut field = Field::default();
        let mut move_list = VecDeque::new();
        let mut position = Point::default();

        for line in self.input.lines() {
            if line.is_empty() {
                continue;
            }
            match &line[0..1] {
                "#" => field.field.push(line.chars().collect::<Vec<char>>()),
                "<" | ">" | "v" | "^" => move_list = line.chars().collect::<VecDeque<char>>(),
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
                move_list,
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

impl WareHouse {
    fn sum_gps(&self) -> usize {
        let mut sum = 0;
        for (idy, line) in self.floor.field.iter().enumerate() {
            for (idx, tile) in line.iter().enumerate() {
                if *tile == 'O' {
                    sum += (idy * 100) + idx;
                }
            }
        }
        sum
    }
    fn do_the_robot(&mut self) {}
    fn get_moves(
        &self,
        dir: &Point,
        current_tile: &Point,
        moves: Vec<Point>,
    ) -> Option<Vec<Point>> {
    }
    fn get_next_tile(&self, dir: &char, current_tile: &Point) -> Point {
        match *dir {
            '^' => *current_tile + Point::NORTH,
            'v' => *current_tile + Point::SOUTH,
            '>' => *current_tile + Point::EAST,
            '<' => *current_tile + Point::WEST,
            _ => panic!("not a valid character"),
        }
    }
    fn process_moves() {}
    fn make_move() {}
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
