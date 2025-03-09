use crate::support::field_tools::{NumField, Point};
use std::{str::FromStr, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> TrailMap {
        let map = NumField::from_str(&self.input).unwrap();
        let mut heads = Vec::with_capacity(100);
        let directions = vec![Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST];
        for (idy, line) in map.field.iter().enumerate() {
            for (idx, num) in line.iter().enumerate() {
                if *num == 0 {
                    heads.push(Point::from((idx, idy)));
                }
            }
        }
        TrailMap {
            map,
            heads,
            directions,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TrailMap {
    map: NumField,
    heads: Vec<Point>,
    directions: Vec<Point>,
}

impl TrailMap {
    fn solve_part_1(&self) -> usize {
        let mut acc = 0;
        for head in &self.heads {
            let mut temp = self.explore(head);
            self.clean_vector(&mut temp);
            acc += temp.len();
        }
        acc
    }

    fn solve_part_2(&self) -> usize {
        let mut acc = 0;
        for head in &self.heads {
            let temp = self.explore(head);
            acc += temp.len();
        }
        acc
    }

    fn explore(&self, point: &Point) -> Vec<Point> {
        let mut acc = Vec::with_capacity(20);
        let current = match self.map.get_point(&point) {
            Some(point) => point,
            None => panic!("Attempt to access point outside field"),
        };
        if current == 9 {
            acc.push(*point);
            return acc;
        }
        let mut to_explore: Vec<Point> = Vec::with_capacity(3);
        for &dir in &self.directions {
            let next = dir + *point;
            match self.map.get_point(&next) {
                Some(val) => {
                    if current + 1 == val {
                        to_explore.push(next);
                    }
                }
                None => continue,
            }
        }
        if to_explore.is_empty() {
            return acc;
        }

        for next in to_explore {
            acc.append(&mut self.explore(&next));
        }
        acc
    }

    fn clean_vector(&self, vector: &mut Vec<Point>) {
        vector.sort_unstable();
        vector.dedup();
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();

    let map = input.parse();

    println!("Part one: {}", map.solve_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let map = input.parse();

    println!("Part two: {}", map.solve_part_2());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day ten answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
