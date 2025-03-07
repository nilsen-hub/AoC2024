use crate::support::field_tools::Point;
use crate::support::field_tools::{self, Field};
use std::{str::FromStr, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn build_part_one_solver(&self) -> PartOneSolver {
        let field = match field_tools::Field::from_str(&self.input) {
            Ok(field) => PartOneSolver {
                field,
                directions: vec![
                    Point::NORTH,
                    Point::SOUTH,
                    Point::EAST,
                    Point::WEST,
                    Point::NORTH_EAST,
                    Point::NORTH_WEST,
                    Point::SOUTH_EAST,
                    Point::SOUTH_WEST,
                ],
            },
            Err(err) => panic!("Error detected: {}", err),
        };
        return field;
    }

    fn build_part_two_solver(&self) -> PartTwoSolver {
        let field = match field_tools::Field::from_str(&self.input) {
            Ok(field) => PartTwoSolver { field },
            Err(err) => panic!("Error detected: {}", err),
        };
        return field;
    }
}

struct PartTwoSolver {
    field: Field,
}

impl PartTwoSolver {
    fn check_xmas(&self, point: Point) -> bool {
        let field = &self.field.field;

        let pne = point + Point::NORTH_EAST;
        let psw = point + Point::SOUTH_WEST;
        let forward = (
            field[pne.y as usize][pne.x as usize],
            field[psw.y as usize][psw.x as usize],
        );

        match forward {
            ('M', 'S') | ('S', 'M') => (),
            _ => return false,
        };

        let pnw = point + Point::NORTH_WEST;
        let pse = point + Point::SOUTH_EAST;
        let backward = (
            field[pnw.y as usize][pnw.x as usize],
            field[pse.y as usize][pse.x as usize],
        );

        match backward {
            ('M', 'S') | ('S', 'M') => (),
            _ => return false,
        };

        true
    }

    fn solve(&self) -> usize {
        let mut acc = 0;
        for (idy, line) in self.field.field.iter().enumerate() {
            for (idx, char) in line.iter().enumerate() {
                if *char == 'A'
                    && (1..(self.field.width - 1)).contains(&(idx as isize))
                    && (1..(self.field.height - 1)).contains(&(idy as isize))
                {
                    if self.check_xmas(Point::from(idx as isize, idy as isize)) {
                        acc += 1;
                    }
                }
            }
        }
        acc
    }
}

struct PartOneSolver {
    field: Field,
    directions: Vec<Point>,
}

impl PartOneSolver {
    fn solve(&self) -> u32 {
        let mut output = 0;
        for (idy, line) in self.field.field.iter().enumerate() {
            for (idx, c) in line.iter().enumerate() {
                if *c == 'X' {
                    output += self.check_xmas(Point::from(idx as isize, idy as isize));
                }
            }
        }
        return output;
    }

    fn check_xmas(&self, point: Point) -> u32 {
        let mut output = 0;
        let target = vec!['M', 'A', 'S'];
        'outer: for direction in &self.directions {
            let mut explore = point;
            if !self.bound_check(&point, &direction, target.len() as isize) {
                continue;
            }
            for index in 0..target.len() {
                explore += *direction;
                if self.field.field[explore.y as usize][explore.x as usize] != target[index] {
                    continue 'outer;
                }
            }
            output += 1;
        }
        output
    }

    fn bound_check(&self, point: &Point, direction: &Point, distance: isize) -> bool {
        let check = *point + (*direction * (distance, distance));
        if (0..self.field.width).contains(&check.x) && (0..self.field.height).contains(&check.y) {
            return true;
        }
        false
    }
}
fn part_1(data: &InputData) {
    let now = Instant::now();
    let field = data.build_part_one_solver();
    println!("Part one: {}", field.solve());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(data: &InputData) {
    let now = Instant::now();
    let solver = data.build_part_two_solver();
    println!("Part two: {}", solver.solve());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day four answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
