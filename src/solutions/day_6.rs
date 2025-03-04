use crate::support::field_tools::Field;
use crate::support::field_tools::Point;
use std::str::FromStr;
use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> Lab {
        let floorplan = Field::from_str(&self.input).unwrap();

        for (idy, line) in floorplan.field.iter().enumerate() {
            for (idx, char) in line.iter().enumerate() {
                if *char == '^' {
                    let guard = Guard {
                        position: Point::from(idx as isize, idy as isize),
                        direction: 0,
                    };
                    return Lab { floorplan, guard };
                }
            }
        }
        return Lab::default();
    }
}

#[derive(Debug, Clone, Default)]
struct Lab {
    floorplan: Field,
    guard: Guard,
}

impl Lab {
    fn solve_part_one(&self) -> usize {
        self.path_recorder().len()
    }
    fn solve_part_two(&mut self) -> usize {
        let mut acc = 0;
        let mut path = self.path_recorder();
        loop {
            let current = match path.pop() {
                Some(point) => point,
                None => break,
            };
            self.floorplan.field[current.y as usize][current.x as usize] = '#';
            if self.is_infinite() {
                acc += 1;
            }
            self.floorplan.field[current.y as usize][current.x as usize] = '.';
        }
        acc
    }
    fn is_infinite(&self) -> bool {
        let mut states: Vec<Guard> = Vec::with_capacity(2000);
        let mut guard = self.guard.clone();
        loop {
            let next = match guard.direction {
                0 => guard.position + Point::NORTH,
                1 => guard.position + Point::EAST,
                2 => guard.position + Point::SOUTH,
                3 => guard.position + Point::WEST,
                _ => panic!(),
            };
            if !self.floorplan.is_in_bounds(&next) {
                return false;
            }
            match self.floorplan.field[next.y as usize][next.x as usize] {
                '.' | '^' => guard.position = next,
                '#' => {
                    if guard.direction < 3 {
                        guard.direction += 1;
                    } else {
                        guard.direction = 0;
                    }
                    if states.contains(&guard) {
                        break;
                    }
                    states.push(guard.clone());
                }
                _ => panic!(),
            };
        }
        true
    }
    fn path_recorder(&self) -> Vec<Point> {
        let mut steps: Vec<Point> = Vec::with_capacity(2000);
        let mut guard = self.guard.clone();
        loop {
            steps.push(guard.position);
            let next = match guard.direction {
                0 => guard.position + Point::NORTH,
                1 => guard.position + Point::EAST,
                2 => guard.position + Point::SOUTH,
                3 => guard.position + Point::WEST,
                _ => panic!(),
            };
            if !self.floorplan.is_in_bounds(&next) {
                break;
            }
            match self.floorplan.field[next.y as usize][next.x as usize] {
                '.' | '^' => guard.position = next,
                '#' => {
                    if guard.direction < 3 {
                        guard.direction += 1;
                    } else {
                        guard.direction = 0;
                    }
                }
                _ => panic!(),
            };
        }
        steps.sort_unstable();
        steps.dedup();
        steps
    }
}

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
struct Guard {
    position: Point,
    direction: usize,
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let parsed = input.parse();

    println!("Part one: {}", parsed.solve_part_one());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut parsed = input.parse();

    println!("Part two: {}", parsed.solve_part_two());
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
