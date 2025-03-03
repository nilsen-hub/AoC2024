use crate::support::field_tools::Point;
use crate::support::field_tools::{self, Field};
use std::ops::Index;
use std::{fs::read_to_string, str::FromStr, time::Instant};

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
            Ok(field) => PartTwoSolver {
                field,
                search_directions: vec![
                    Point::NORTH_EAST,
                    Point::NORTH_WEST,
                    Point::SOUTH_EAST,
                    Point::SOUTH_WEST,
                ],
                check_directions: vec![Point::NORTH, Point::SOUTH, Point::EAST, Point::WEST],
            },
            Err(err) => panic!("Error detected: {}", err),
        };
        return field;
    }
}
struct PartTwoSolver {
    field: Field,
    search_directions: Vec<Point>,
    check_directions: Vec<Point>,
}

impl PartTwoSolver {
    fn check_xmas(&self, point: Point) -> usize {
        let mut acc = 0;
        let target = vec!['A', 'S'];
        'outer: for (num, direction) in self.search_directions.iter().enumerate() {
            let mut explore = point;
            if !self.bound_check(&point, direction, target.len() as isize) {
                continue;
            }
            for index in 0..target.len() {
                explore += *direction;

                if self.field.field[explore.y as usize][explore.x as usize] != target[index] {
                    continue 'outer;
                }

                match num {
                    0 => {
                        if self.field.field[(point.y + (Point::NORTH.y * 2)) as usize]
                            [point.x as usize]
                            == 'M'
                            && self.field.field[point.y as usize]
                                [(point.x + (Point::EAST.x * 2)) as usize]
                                == 'S'
                            || self.field.field[(point.y + (Point::NORTH.y * 2)) as usize]
                                [point.x as usize]
                                == 'S'
                                && self.field.field[point.y as usize]
                                    [(point.x + (Point::EAST.x * 2)) as usize]
                                    == 'M'
                        {
                            acc += 1;
                        }
                    }
                    1 => {
                        if self.field.field[(point.y + (Point::NORTH.y * 2)) as usize]
                            [point.x as usize]
                            == 'M'
                            && self.field.field[point.y as usize]
                                [(point.x + (Point::WEST.x * 2)) as usize]
                                == 'S'
                            || self.field.field[(point.y + (Point::NORTH.y * 2)) as usize]
                                [point.x as usize]
                                == 'S'
                                && self.field.field[point.y as usize]
                                    [(point.x + (Point::WEST.x * 2)) as usize]
                                    == 'M'
                        {
                            acc += 1;
                        }
                    }
                    2 => {
                        if self.field.field[(point.y + (Point::SOUTH.y * 2)) as usize]
                            [point.x as usize]
                            == 'M'
                            && self.field.field[point.y as usize]
                                [(point.x + (Point::EAST.x * 2)) as usize]
                                == 'S'
                            || self.field.field[(point.y + (Point::SOUTH.y * 2)) as usize]
                                [point.x as usize]
                                == 'S'
                                && self.field.field[point.y as usize]
                                    [(point.x + (Point::EAST.x * 2)) as usize]
                                    == 'M'
                        {
                            acc += 1;
                        }
                    }
                    3 => {
                        if self.field.field[(point.y + (Point::SOUTH.y * 2)) as usize]
                            [point.x as usize]
                            == 'M'
                            && self.field.field[point.y as usize]
                                [(point.x + (Point::WEST.x * 2)) as usize]
                                == 'S'
                            || self.field.field[(point.y + (Point::SOUTH.y * 2)) as usize]
                                [point.x as usize]
                                == 'S'
                                && self.field.field[point.y as usize]
                                    [(point.x + (Point::WEST.x * 2)) as usize]
                                    == 'M'
                        {
                            acc += 1;
                        }
                    }
                    _ => panic!("check xmas part two is really messed up"),
                }
            }
        }
        acc
    }

    fn solve(&self) -> usize {
        let mut acc = 0;
        for (idy, line) in self.field.field.iter().enumerate() {
            for (idx, char) in line.iter().enumerate() {
                if *char == 'M' {
                    acc += self.check_xmas(Point::from(idx as isize, idy as isize));
                }
            }
        }
        acc / 2
    }
    fn bound_check(&self, point: &Point, direction: &Point, distance: isize) -> bool {
        let check = *point + (*direction * (distance, distance));
        if (0..self.field.width).contains(&check.x) && (0..self.field.height).contains(&check.y) {
            return true;
        }
        false
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
fn part_2_deprec(data: &InputData) {
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
