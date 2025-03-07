use std::time::Instant;

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> Vec<&str> {
        let mul_filter = self.input.split("mul(").collect::<Vec<&str>>();
        return mul_filter;
    }

    fn parse_part_2(&self) -> Vec<&str> {
        let mut output: Vec<&str> = Vec::with_capacity(200);
        let mut collector: Vec<&str> = Vec::with_capacity(200);
        let do_filter = self.input.split("do()").collect::<Vec<&str>>();

        for line in do_filter {
            let temp = line.split("don't").collect::<Vec<&str>>();
            collector.push(temp[0]);
        }

        for line in collector {
            let temp = line.split("mul(").collect::<Vec<&str>>();
            for el in temp {
                output.push(el);
            }
        }

        output
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let parsed = input.parse_part_1();
    let data = find_muls(parsed);

    for result in data {
        acc += result.0 * result.1;
    }

    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let parsed = input.parse_part_2();
    let data = find_muls(parsed);

    for result in data {
        acc += result.0 * result.1;
    }

    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn find_muls(data: Vec<&str>) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::with_capacity(500);

    for line in data {
        let comma_filter = line.split(",").collect::<Vec<&str>>();

        if comma_filter.len() == 1 {
            continue;
        }

        let left = match comma_filter[0].parse::<usize>() {
            Ok(number) => number,
            Err(_error) => continue,
        };

        let end_filter = comma_filter[1].split(")").collect::<Vec<&str>>();

        let right = match end_filter[0].parse::<usize>() {
            Ok(number) => number,
            Err(_error) => continue,
        };

        output.push((left, right));
    }

    output
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day three answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
