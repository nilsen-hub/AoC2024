use crate::support::field_tools::Point;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
struct Bathroom {
    size: Point,
    quad: [Quadrant; 4],
    robots: Vec<Robot>,
}

#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point,
    vel: Point,
}

#[derive(Debug, Clone, Default, Copy)]
struct Quadrant {
    perimeter: (Point, Point), // upper left corner, lower right corner
    robots: isize,
}

impl Quadrant {
    fn get_quad(&mut self, id: usize, size: Point) {
        match id {
            0 => {
                self.perimeter = (
                    Point::from((0, 0)),
                    Point::from((size.x / 2 - 1, size.y / 2 - 1)),
                )
            }
            1 => {
                self.perimeter = (
                    Point::from((size.x / 2 + 1, 0)),
                    Point::from((size.x - 1, size.y / 2 - 1)),
                )
            }
            2 => {
                self.perimeter = (
                    Point::from((0, size.y / 2 + 1)),
                    Point::from((size.x / 2 - 1, size.y - 1)),
                )
            }
            3 => {
                self.perimeter = (
                    Point::from((size.x / 2 + 1, size.y / 2 + 1)),
                    Point::from((size.x - 1, size.y - 1)),
                )
            }
            _ => panic!("should never happen"),
        };
    }

    fn detect_robot(&mut self, robot: &Robot) -> bool {
        let range_x = self.perimeter.0.x..=self.perimeter.1.x;
        let range_y = self.perimeter.0.y..=self.perimeter.1.y;
        if range_x.contains(&robot.pos.x) && range_y.contains(&robot.pos.y) {
            return true;
        }
        false
    }
}
struct InputData {
    input: String,
    size: Point,
}

impl InputData {
    fn parse_part_1(&self) {
        let mut robots = Vec::with_capacity(400);
        for line in self.input.lines() {
            robots.push(self.build_robot(line));
        }
    }

    fn build_robot(&self, line: &str) -> Robot {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let pos_v = split[0]
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let vel_v = split[0]
            .strip_prefix("v=")
            .unwrap()
            .split(",")
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        Robot {
            pos: Point::from((pos_v[0], pos_v[1])),
            vel: Point::from((vel_v[0], vel_v[1])),
        }
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
        size: Point::from((101, 103)),
    };

    println!("Day fourteen answers:");
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
