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
                        direction: Dir::default(),
                    };
                    return Lab {
                        floorplan,
                        guard,
                        path_map: Vec::with_capacity(5500),
                    };
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
    path_map: Vec<Point>,
}

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
enum Dir {
    #[default]
    North,
    East,
    South,
    West,
}
impl Lab {
    fn solve_part_one(&mut self) -> usize {
        self.path_engine();
        self.path_map.len()
    }

    fn path_engine(&mut self) {
        self.path_map.push(self.guard.position);
        loop {
            let mut turn = self.find_next_turn();
            self.path_recorder(&turn);
            if !self.floorplan.is_in_bounds(&turn) {
                break;
            }
            turn += match self.guard.direction {
                Dir::North => Point::SOUTH,
                Dir::East => Point::WEST,
                Dir::South => Point::NORTH,
                Dir::West => Point::EAST,
            };
            self.guard.position = turn;
            self.guard.turn();
        }
        self.path_map.sort_unstable();
        self.path_map.dedup();
    }

    fn solve_part_two(&mut self) -> usize {
        let mut acc = 0;
        self.path_engine();
        for path in &self.path_map {
            //println!("path: {:?}", path);
        }
        loop {
            let current = match self.path_map.pop() {
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
    fn find_next_turn(&self) -> Point {
        let mut next = self.guard.position;
        loop {
            next += match self.guard.direction {
                Dir::North => Point::NORTH,
                Dir::East => Point::EAST,
                Dir::South => Point::SOUTH,
                Dir::West => Point::WEST,
            };
            if !self.floorplan.is_in_bounds(&next) {
                break;
            }
            match self.floorplan.field[next.y as usize][next.x as usize] {
                '.' | '^' => continue,
                '#' => break,
                _ => panic!(),
            };
        }
        next
    }

    fn is_infinite(&self) -> bool {
        let mut states: Vec<Guard> = Vec::with_capacity(2000);
        let mut guard = self.guard.clone();
        loop {
            let next = match guard.direction {
                Dir::North => guard.position + Point::NORTH,
                Dir::East => guard.position + Point::EAST,
                Dir::South => guard.position + Point::SOUTH,
                Dir::West => guard.position + Point::WEST,
            };
            if !self.floorplan.is_in_bounds(&next) {
                return false;
            }
            match self.floorplan.field[next.y as usize][next.x as usize] {
                '.' | '^' => guard.position = next,
                '#' => {
                    guard.turn();
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
    fn path_recorder(&mut self, pos: &Point) {
        let mut pos = *pos;
        loop {
            if pos == self.guard.position {
                return;
            }
            pos += match self.guard.direction {
                Dir::North => Point::SOUTH,
                Dir::East => Point::WEST,
                Dir::South => Point::NORTH,
                Dir::West => Point::EAST,
            };
            self.path_map.push(pos);
        }
    }
    fn path_recorder_deprec(&self) -> Vec<Point> {
        let mut steps: Vec<Point> = Vec::with_capacity(2000);
        let mut guard = self.guard.clone();
        loop {
            steps.push(guard.position);
            let next = match guard.direction {
                Dir::North => guard.position + Point::NORTH,
                Dir::East => guard.position + Point::EAST,
                Dir::South => guard.position + Point::SOUTH,
                Dir::West => guard.position + Point::WEST,
            };
            if !self.floorplan.is_in_bounds(&next) {
                break;
            }
            match self.floorplan.field[next.y as usize][next.x as usize] {
                '.' | '^' => guard.position = next,
                '#' => guard.turn(),
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
    direction: Dir,
}

impl Guard {
    fn turn(&mut self) {
        match self.direction {
            Dir::North => self.direction = Dir::East,
            Dir::East => self.direction = Dir::South,
            Dir::South => self.direction = Dir::West,
            Dir::West => self.direction = Dir::North,
        }
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut parsed = input.parse();

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
