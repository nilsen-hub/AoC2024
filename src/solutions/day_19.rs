use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> TowelConstructor {
        let mut towel_constructor = TowelConstructor::default();
        for (index, line) in self.input.lines().into_iter().enumerate() {
            if index == 0 {
                let patterns = line
                    .split_whitespace()
                    .collect::<String>()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                for pattern in patterns {
                    let pattern = pattern.chars().collect::<Vec<char>>();
                    towel_constructor
                        .patterns
                        .entry(pattern[0].clone())
                        .and_modify(|v| v.push(pattern.clone()))
                        .or_insert(vec![pattern]);
                }
                continue;
            }
            if line.is_empty() {
                continue;
            }
            towel_constructor.designs.push(line.chars().collect());
        }

        towel_constructor
    }

    fn parse_part_2(&self) {}
}
#[derive(Debug, Default, Clone)]
struct TowelConstructor {
    patterns: HashMap<char, Vec<Vec<char>>>,
    designs: Vec<Vec<char>>,
}

impl TowelConstructor {
    fn solve_part_1(&mut self) -> usize {
        let mut acc = 0;
        let mut counter = 1;
        self.sort_patterns();
        self.designs.reverse();
        loop {
            let design = match self.designs.pop() {
                Some(d) => d,
                None => break,
            };
            if self.build_towel(design.clone(), design.clone(), Vec::new()) {
                acc += 1;
                println!("POSSIBLE design: {}", counter);
            } else {
                println!("NOT POSSIBLE design: {}", counter);
            }
            counter += 1;
        }
        acc
    }
    fn sort_patterns(&mut self) {
        for group in self.patterns.
    }
    fn build_towel(&self, design: Vec<char>, target: Vec<char>, current: Vec<char>) -> bool {
        let patterns = self.patterns.clone();
        let mut design = design.clone();
        let mut current = current.clone();
        for pattern in patterns {
            let to_match_iter = design.windows(pattern.len());
            let to_match = match to_match_iter.clone().next() {
                Some(v) => v.to_vec(),
                None => continue,
            };
            if pattern == to_match {
                current.append(&mut pattern.clone());
                if current == target {
                    return true;
                }
                design.drain(0..pattern.len());
                if self.build_towel(design.clone(), target.clone(), current.clone()) {
                    return true;
                }
            }
        }
        false
    }
}
fn part_1(input: &InputData) {
    let now = Instant::now();

    let mut towel_constructor = input.parse_part_1();

    println!("Part one: {}", towel_constructor.solve_part_1());
    println!("Runtime (seconds): {}", now.elapsed().as_secs());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let parsed = input.parse_part_2();

    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str, test_data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    let test = InputData {
        input: test_data.to_string(),
    };

    println!("Day nineteen answers:");
    println!("");

    //part_1(&test);
    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_works() {}
}
