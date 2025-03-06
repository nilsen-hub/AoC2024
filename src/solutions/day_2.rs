use std::time::Instant;

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Running day two");
    println!("");

    println!("Answers:");
    println!("");

    part_1(&input);
    println!("");

    part_2(&input);
    println!("");
}

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> Vec<Vec<i8>> {
        let lines = self.input.lines();
        let mut output: Vec<Vec<i8>> = Vec::with_capacity(1000);

        for line in lines {
            let as_string = line.split_whitespace().collect::<Vec<&str>>();
            let mut conv: Vec<i8> = Vec::with_capacity(15);
            for number in as_string {
                conv.push(number.parse().unwrap());
            }
            output.push(conv);
        }

        output
    }
}

// part one solver
fn report_analyzer(mut report: Vec<i8>) -> bool {
    let last = report.len() - 2;

    if report[0] - report[1] > 0 {
        report.reverse();
    }

    for (index, num) in report.iter().enumerate() {
        if index > last {
            break;
        }
        let step = report[index + 1] - num;
        match step {
            1..4 => continue,
            _ => return false,
        }
    }

    true
}

// part two solver
fn dampened_report_analyzer(mut report: Vec<i8>) -> bool {
    let last = report.len() - 2;
    let mut safe = true;

    if report[0] - report[1] > 0 {
        report.reverse();
    }

    for (index, num) in report.iter().enumerate() {
        if index > last {
            break;
        }
        let step = report[index + 1] - num;
        match step {
            1..4 => continue,
            _ => {
                if safe == false {
                    return false;
                } else {
                    safe = false;
                }
            }
        }
    }

    true
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let mut parsed = input.parse();

    loop {
        match parsed.pop() {
            Some(report) => {
                if report_analyzer(report) {
                    acc += 1;
                }
            }
            None => break,
        }
    }
    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let mut parsed = input.parse();

    loop {
        match parsed.pop() {
            Some(report) => {
                if dampened_report_analyzer(report) {
                    acc += 1;
                }
            }
            None => break,
        }
    }
    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_analyzer_works() {
        let test_ascent: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7];
        let test_descent: Vec<i8> = vec![7, 6, 5, 4, 3, 2, 1];
        let test_false_1: Vec<i8> = vec![1, 2, 2, 3, 4, 5, 6];
        let test_false_2: Vec<i8> = vec![1, 2, 6, 8, 9];

        assert_eq!(report_analyzer(test_ascent), true);
        assert_eq!(report_analyzer(test_descent), true);
        assert_eq!(report_analyzer(test_false_1), false);
        assert_eq!(report_analyzer(test_false_2), false);
    }
}
