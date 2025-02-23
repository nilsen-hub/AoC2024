use std::{collections::HashMap, fs::read_to_string, time::Instant};

pub fn solution(path: &str) {
    #[derive(Debug, Clone)]
    struct InputData {
        input: String,
    }

    impl InputData {
        fn parse_part_1(&self) -> Part1Data {
            let mut output = Part1Data::default();
            let input = self.input.lines();
            for line in input {
                let parts: Vec<&str> = line.split_whitespace().collect();
                output.left.push(parts[0].parse().unwrap());
                output.right.push(parts[1].parse().unwrap());
            }

            output.left.sort_unstable();
            output.right.sort_unstable();

            output
        }

        fn parse_part_2(&self) -> Part2Data {
            let mut output = Part2Data::default();
            let input = self.input.lines();

            for line in input {
                let parts: Vec<&str> = line.split_whitespace().collect();
                output.left.push(parts[0].parse().unwrap());
                *output.right.entry(parts[1].parse().unwrap()).or_insert(0) += 1;
            }

            output
        }
    }

    #[derive(Debug, Clone, Default)]
    struct Part1Data {
        left: Vec<usize>,
        right: Vec<usize>,
    }

    #[derive(Debug, Clone, Default)]
    struct Part2Data {
        left: Vec<usize>,
        right: HashMap<usize, usize>,
    }

    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };

    println!("Running day one");
    println!("");

    println!("Answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");

    fn part_1(input: &InputData) {
        let now = Instant::now();
        let mut acc: usize = 0;

        let parsed = input.parse_part_1();
        for (index, el) in parsed.left.iter().enumerate() {
            acc += el.abs_diff(parsed.right[index]);
        }
        println!("Part one: {}", acc);
        println!("Runtime (micros): {}", now.elapsed().as_micros());
    }

    fn part_2(input: &InputData) {
        let now = Instant::now();
        let mut acc: usize = 0;

        let parsed = input.parse_part_2();

        for number in parsed.left {
            match parsed.right.get(&number) {
                Some(val) => acc += number * val,
                None => continue,
            }
        }
        println!("Part two: {}", acc);
        println!("Runtime (micros): {}", now.elapsed().as_micros());
    }
}
