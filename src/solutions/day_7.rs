use std::{collections::VecDeque, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> Vec<Equation> {
        let mut output = Vec::with_capacity(850);
        let lines = self.input.lines();
        for line in lines {
            let eq = line.split(':').collect::<Vec<&str>>();
            output.push(Equation {
                target: eq[0].parse().unwrap(),
                source: eq[1]
                    .split_whitespace()
                    .map(|str| str.parse::<usize>().unwrap())
                    .collect(),
            });
        }
        output
    }
}

#[derive(Debug, Clone)]
struct Equation {
    target: usize,
    source: VecDeque<usize>,
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let parsed = input.parse();

    for mut eq in parsed {
        let target = eq.target.clone();
        let first = eq.source.pop_front().unwrap();
        if is_possible_part_one(eq.target, first, eq.source) {
            acc += target;
        }
    }

    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let parsed = input.parse();

    for mut eq in parsed {
        let target = eq.target.clone();
        let first = eq.source.pop_front().unwrap();
        if is_possible_part_two(eq.target, first, eq.source) {
            acc += target;
        }
    }

    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn is_possible_part_one(target: usize, acc: usize, mut source: VecDeque<usize>) -> bool {
    if source.len() == 0 {
        return acc == target;
    }
    let first = source.pop_front().unwrap();

    if is_possible_part_one(target, acc + first, source.clone()) {
        return true;
    }
    if is_possible_part_one(target, acc * first, source) {
        return true;
    }

    return false;
}

fn is_possible_part_two(target: usize, acc: usize, mut source: VecDeque<usize>) -> bool {
    //let mut source = source.clone();
    if source.len() == 0 {
        return acc == target;
    }
    let first = source.pop_front().unwrap();

    if is_possible_part_two(target, acc + first, source.clone()) {
        return true;
    }
    if is_possible_part_two(target, acc * first, source.clone()) {
        return true;
    }
    if is_possible_part_two(target, number_cat(&acc, &first), source) {
        return true;
    }

    return false;
}

fn number_cat(left: &usize, right: &usize) -> usize {
    format!("{}{}", left, right).parse().unwrap()
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day seven answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
