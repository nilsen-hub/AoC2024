use crate::support::field_tools::{Field, Point};
use std::{str::FromStr, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> AntennaMap {
        let field = Field::from_str(&self.input).unwrap();
        let mut antennas: Vec<Vec<Point>> = vec![Vec::new(); 128];
        for (idy, line) in field.field.iter().enumerate() {
            for (idx, c) in line.iter().enumerate() {
                if *c != '.' {
                    antennas[*c as usize].push(Point::from((idx, idy)));
                }
            }
        }
        AntennaMap { field, antennas }
    }
}

struct AntennaMap {
    field: Field,
    antennas: Vec<Vec<Point>>, // not using a HashMap because theyre slow as fuck
                               // indexing on ASCII values is much faster
}

impl AntennaMap {
    fn solve_part_one(&self) -> usize {
        let mut acc: Vec<Point> = Vec::with_capacity(500);

        for antenna_group in self.antennas.clone() {
            if antenna_group.is_empty() {
                continue;
            }
            let bounds = antenna_group.len();
            for (index, point) in antenna_group.iter().enumerate() {
                let mut count = 0;
                loop {
                    if count == bounds {
                        break;
                    }
                    if count == index {
                        count += 1;
                        continue;
                    }
                    let offset = *point - antenna_group[count];
                    let antinode = *point + offset;
                    if self.field.is_in_bounds(&antinode) {
                        acc.push(antinode);
                    }
                    count += 1;
                }
            }
        }

        acc.sort_unstable();
        acc.dedup();
        acc.len()
    }

    fn solve_part_two(&self) -> usize {
        let mut acc: Vec<Point> = Vec::with_capacity(500);

        for antenna_group in self.antennas.clone() {
            if antenna_group.is_empty() {
                continue;
            }
            let bounds = antenna_group.len();
            for (index, point) in antenna_group.iter().enumerate() {
                let mut count = 0;
                loop {
                    if count == bounds {
                        break;
                    }
                    if count == index {
                        count += 1;
                        continue;
                    }
                    let offset = *point - antenna_group[count];
                    let mut antinode = *point + offset;
                    acc.push(*point);
                    loop {
                        if !self.field.is_in_bounds(&antinode) {
                            break;
                        }
                        acc.push(antinode);
                        antinode += offset;
                    }
                    count += 1;
                }
            }
        }

        acc.sort_unstable();
        acc.dedup();
        acc.len()
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let parsed = input.parse();

    println!("Part one: {}", parsed.solve_part_one());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();

    let parsed = input.parse();

    println!("Part two: {}", parsed.solve_part_two());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day eight answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
