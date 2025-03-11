use std::time::Instant;

use crate::support::aoc_math::{gcd, lcm};
type Coords = (usize, usize);

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> Vec<ClawMachine> {
        let mut machines = Vec::with_capacity(500);
        let mut data = self.input.lines();
        loop {
            machines.push(ClawMachine {
                a: self.get_button(data.next().unwrap()),
                b: self.get_button(data.next().unwrap()),
                target: self.get_prize(data.next().unwrap()),
            });

            match data.next() {
                Some(_void) => (),
                None => break,
            }
        }
        machines
    }

    fn get_button(&self, button: &str) -> Coords {
        let split: Vec<&str> = button.split('+').collect();
        (
            split[1]
                .strip_suffix(", Y")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            split[2].parse::<usize>().unwrap(),
        )
    }

    fn get_prize(&self, prize: &str) -> Coords {
        let split: Vec<&str> = prize.split('=').collect();
        (
            split[1]
                .strip_suffix(", Y")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            split[2].parse::<usize>().unwrap(),
        )
    }
}

#[derive(Debug, Clone)]
struct ClawMachine {
    a: Coords,
    b: Coords,
    target: Coords,
}

impl ClawMachine {
    fn find_bxmod_zero(&self, a_presses: usize) -> usize {
        if (self.target.0 - (self.a.0 * a_presses)) % self.b.0 == 0 {
            return a_presses;
        }
        return self.find_bxmod_zero(a_presses + 1);
    }
    fn solve(&self) -> usize {
        let target = self.target;
        let a = self.a;
        let b = self.b;

        // this removes machines that will never reach bx_mod == 0 + some more
        if target.0 % gcd(a.0, b.0) != 0 || target.1 % gcd(a.1, b.1) != 0 {
            return 0;
        }

        // First we figure out how many a presses we need to make the
        // amount of b presses needed fit into mod delta target/x = 0
        let mut a_presses = self.find_bxmod_zero(0);

        // Now that we have our X, we ned to get Y into line.
        // Position holds state of Y-side
        let mut position = (a.0 * a_presses, a.1 * a_presses);

        // Start by calculating how much Y moves pr. LCM derived increment
        let lcm = lcm(a.0, b.0);
        let a_increment = lcm / a.0;
        let b_increment = lcm / b.0;

        let mut b_presses = (target.0 - position.0) / b.0;
        let current_y = (b.1 * b_presses) + position.1;
        let increment_y = (a_increment * a.1).abs_diff(b_increment * b.1);

        // Now that we have the Y increment value, we can test if it fits neatly
        // into the delta between the current y value and the target y value.
        // If it fails, we can discard the Claw Machine.
        let delta_y_target = current_y.abs_diff(target.1);
        if delta_y_target % increment_y != 0 {
            return 0;
        }

        // To get the solution, we need to know how many a presses are needed
        // to reach our target Y, first the a presses, luckily this is quite simple:
        let iter_to_go = delta_y_target / increment_y;
        a_presses += a_increment * iter_to_go;

        // then update the position to figure out how many b presses are needed
        // we could calculate b presses directly, but I think the added clarity
        // outwheighs the cost of the extra step
        position.0 = a.0 * a_presses;
        b_presses = (target.0 - position.0) / b.0;
        // then we return the answer
        return (a_presses * 3) + b_presses;
    }
}
fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let machines = input.parse();

    for machine in machines {
        acc += machine.solve();
    }

    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;
    let machines = input.parse();

    for mut machine in machines {
        machine.target.0 += 10000000000000;
        machine.target.1 += 10000000000000;
        acc += machine.solve();
    }

    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day thirteen answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
