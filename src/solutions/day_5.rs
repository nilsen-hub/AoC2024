use std::{collections::HashMap, fs::read_to_string, time::Instant};
// 5.2 time to beat 10ms
#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> Updates {
        let mut page_map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(1200);
        let mut orders: Vec<Vec<usize>> = Vec::with_capacity(256);
        let lines = self.input.lines();

        for line in lines {
            if line.contains('|') {
                let rule = line.split('|').collect::<Vec<&str>>();
                page_map
                    .entry(rule[1].parse().unwrap())
                    .and_modify(|e| e.push(rule[0].parse().unwrap()))
                    .or_insert(vec![rule[0].parse().unwrap()]);
                continue;
            }
            if line.contains(',') {
                orders.push(
                    line.split(',')
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<usize>>(),
                );
            }
        }
        return Updates { page_map, orders };
    }

    fn parse_part_2(&self) {}
}
#[derive(Debug, Clone)]
struct Updates {
    page_map: HashMap<usize, Vec<usize>>,
    orders: Vec<Vec<usize>>,
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

pub fn solution(path: &str) {
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };

    println!("Running day five");
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
