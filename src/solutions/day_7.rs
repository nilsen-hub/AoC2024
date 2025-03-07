use std::time::Instant;

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> Vec<Equation> {
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

    fn parse_part_2(&self) {}
}

#[derive(Debug, Clone)]
struct Equation {
    target: usize,
    source: Vec<usize>,
}

impl Equation {
    fn is_possible() {}
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_analysis() {}
}
