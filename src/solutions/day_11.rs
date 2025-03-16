use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> HashMap<usize, usize> {
        let mut output = HashMap::new();
        let split: Vec<usize> = self
            .input
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        for num in split {
            output
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        output
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let stones = input.parse();

    println!("Part one: {}", blink_machine(stones, 25));
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let stones = input.parse();

    println!("Part two: {}", blink_machine(stones, 75));
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn blink_machine(input: HashMap<usize, usize>, limit: usize) -> usize {
    let mut input_map = input;
    let mut limit = limit;
    while limit > 0 {
        let mut temp_map: HashMap<usize, usize> = HashMap::with_capacity(4000);
        for (stone, amount) in input_map {
            if stone == 0 {
                temp_map
                    .entry(1)
                    .and_modify(|count| *count += amount)
                    .or_insert(amount);
                continue;
            }
            let len = stone.checked_ilog10().unwrap_or(0) + 1;
            if len & 1 == 0 {
                let divisor = 10_usize.pow(len / 2);
                temp_map
                    .entry(stone / divisor)
                    .and_modify(|count| *count += amount)
                    .or_insert(amount);
                temp_map
                    .entry(stone % divisor)
                    .and_modify(|count| *count += amount)
                    .or_insert(amount);
                continue;
            }
            temp_map
                .entry(stone * 2024)
                .and_modify(|count| *count += amount)
                .or_insert(amount);
        }
        input_map = temp_map;
        limit -= 1;
    }
    let acc = input_map.values().sum();
    acc
}

pub fn solution(data: &str, _test_data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day eleven answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
