use std::time::Instant;

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) {
        let mut field_string: String = String::default();
        let mut directions = "placeholder";
        for line in self.input.lines() {
            match &line[0..1] {
                "#" => field_string.push_str(line),
                "<" | ">" | "v" | "^" => directions = line,
                _ => continue,
            }
        }

        for line in field_string.lines() {
            println!("{}", line);
        }
        println!("");
        println!("{}", directions);
    }

    fn parse_part_2(&self) {}
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

    println!("Day fifteen answers:");
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
