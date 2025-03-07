use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> Updates {
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
}
#[derive(Debug, Clone)]
struct Updates {
    page_map: HashMap<usize, Vec<usize>>,
    orders: Vec<Vec<usize>>,
}

impl Updates {
    fn solve_part_one(&self) -> usize {
        let mut acc = 0;

        for order in &self.orders {
            if self.is_valid(&order) {
                acc += order[order.len() / 2];
            }
        }

        acc
    }

    fn solve_part_two(&self) -> usize {
        let mut acc = 0;

        for mut order in self.orders.clone() {
            if !self.is_valid(&order) {
                order = self.make_valid(order);
                acc += order[order.len() / 2];
            }
        }

        acc
    }

    fn make_valid(&self, mut order: Vec<usize>) -> Vec<usize> {
        let bound = order.len();
        let mut order_counter = 0;

        loop {
            if order_counter == bound {
                if self.is_valid(&order) {
                    return order;
                }
                order_counter = 0;
                continue;
            }

            let page = order[order_counter];
            let map = match self.page_map.get(&page) {
                Some(out) => out,
                None => {
                    order_counter += 1;
                    continue;
                }
            };

            let mut count = order_counter + 1;
            while count < bound {
                if map.contains(&order[count]) {
                    let swap = order[order_counter];
                    order[order_counter] = order[count];
                    order[count] = swap;

                    if self.is_valid(&order) {
                        return order;
                    }

                    order_counter = 0;
                    break;
                }
                count += 1;
            }
            order_counter += 1;
        }
    }

    fn is_valid(&self, order: &Vec<usize>) -> bool {
        for (index, page) in order.iter().enumerate() {
            let map = match self.page_map.get(page) {
                Some(out) => out,
                None => continue,
            };
            let mut count = index;
            while count < order.len() {
                if map.contains(&order[count]) {
                    return false;
                }
                count += 1;
            }
        }
        true
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

    println!("Day five answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
