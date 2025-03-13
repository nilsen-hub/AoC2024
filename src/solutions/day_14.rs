use crate::support::{
    //aoc_qol::clear_terminal,
    field_tools::{Field, Point},
};
use std::time::Instant;

#[derive(Debug, Clone, Default)]
struct Display {
    robots: Vec<Robot>,
    screen: Field<char>,
    possible_tree: bool,
    line_to_check: usize,
    room_size: Point,
}

impl Display {
    fn solve_part_2(&mut self) -> isize {
        let mut counter = 0;
        let mut increment = 1;
        loop {
            if self.possible_tree {
                increment = self.screen.height;
                self.frame_buffer();
                if line_checker(&self.screen.field[self.line_to_check]) {
                    //self.draw();
                    break;
                }
                self.possible_tree = false;
            }
            counter += increment;
            self.get_next_frame(increment);
        }

        counter
    }

    fn clear(&mut self) {
        let scan_line = vec![' '; self.screen.width as usize];
        self.screen.field = vec![scan_line; self.screen.height as usize];
    }
    fn get_next_frame(&mut self, increment: isize) {
        let mut index = 0;
        let mut star_counter = vec![Point::default(); self.room_size.y as usize];
        loop {
            self.robots[index].move_robot(self.room_size, &increment);
            let pos = self.robots[index].pos.y;
            star_counter[pos as usize].x = pos;
            star_counter[pos as usize].y += 1;
            index += 1;
            if index == self.robots.len() {
                star_counter.sort_by_key(|point| point.y);
                let last = star_counter.last().unwrap();
                if last.y >= 30 {
                    self.possible_tree = true;
                    self.line_to_check = last.x as usize;
                }
                break;
            }
        }
    }
    fn frame_buffer(&mut self) {
        self.clear();
        for robot in &self.robots {
            self.screen.field[robot.pos.y as usize][robot.pos.x as usize] = '*';
        }
    }
    //fn draw(&self) {
    //    clear_terminal();
    //    for (index, line) in self.screen.field.iter().enumerate() {
    //        for c in line {
    //            print!("{c}");
    //        }
    //        if index == 70 {
    //            break;
    //        }
    //        println!("");
    //    }
    //    println!("");
    //}
}

#[derive(Debug, Clone, Default)]
struct Bathroom {
    size: Point,
    quad: [Quadrant; 4],
    robots: Vec<Robot>,
}

impl Bathroom {
    fn place_robot(&mut self, robot: &Robot) {
        let mut index = 0;
        loop {
            if self.quad[index].detect_robot(robot) {
                self.quad[index].robots += 1;
                break;
            }
            index += 1;
            if index == 4 {
                break;
            }
        }
    }

    fn solve_part_1(&mut self) -> isize {
        let mut acc = 1;
        let steps = 100;
        for mut robot in self.robots.clone() {
            robot.move_robot(self.size, &steps);
            self.place_robot(&robot);
        }
        for quad in self.quad {
            if quad.robots > 0 {
                acc *= quad.robots;
            }
        }
        acc
    }
}

#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn move_robot(&mut self, room_size: Point, steps: &isize) {
        self.pos.x = (self.pos.x + (self.vel.x * steps)).rem_euclid(room_size.x);
        self.pos.y = (self.pos.y + (self.vel.y * steps)).rem_euclid(room_size.y);
    }
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
    fn parse_part_1(&self) -> Bathroom {
        let mut robots = Vec::with_capacity(400);
        for line in self.input.lines() {
            robots.push(self.build_robot(line));
        }
        let mut bathroom = Bathroom::default();
        bathroom.size = self.size;
        bathroom.robots = robots;
        for index in 0..4 as usize {
            bathroom.quad[index].get_quad(index, self.size);
        }

        bathroom
    }

    fn build_robot(&self, line: &str) -> Robot {
        let split = line.split_whitespace().collect::<Vec<&str>>();

        let pos_v = split[0]
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        let vel_v = split[1]
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

    fn parse_part_2(&self) -> Display {
        let mut robots = Vec::with_capacity(400);
        let scan_line = vec![' '; self.size.x as usize];
        let screen = Field {
            field: vec![scan_line; self.size.y as usize],
            width: self.size.x,
            height: self.size.y,
        };

        for line in self.input.lines() {
            robots.push(self.build_robot(line));
        }

        Display {
            robots,
            screen,
            possible_tree: false,
            line_to_check: 0,
            room_size: self.size,
        }
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut bathroom = input.parse_part_1();

    println!("Part one: {}", bathroom.solve_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut display = input.parse_part_2();

    println!("Part two: {}", display.solve_part_2());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn line_checker(line: &Vec<char>) -> bool {
    let mut window = line.windows(30);
    'outer: loop {
        let view = match window.next() {
            Some(view) => view,
            None => return false,
        };
        for el in view {
            if *el != '*' {
                continue 'outer;
            }
        }
        return true;
    }
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
