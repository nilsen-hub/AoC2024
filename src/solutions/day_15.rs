use crate::support::field_tools::{Field, Point};
use std::{collections::VecDeque, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> WareHouse {
        let mut field = Field::default();
        let mut move_list = VecDeque::new();

        for line in self.input.lines() {
            if line.is_empty() {
                continue;
            }
            match &line[0..1] {
                "#" => field.field.push(line.chars().collect::<Vec<char>>()),
                "<" | ">" | "v" | "^" => move_list = line.chars().collect::<VecDeque<char>>(),
                _ => continue,
            }
        }

        field.width = field.field[0].len() as isize;
        field.height = field.field.len() as isize;

        WareHouse {
            floor: field,
            robot: Robot {
                position: Point::default(),
                move_list,
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
struct WareHouse {
    floor: Field<char>,
    robot: Robot,
}

impl WareHouse {
    fn sum_gps_part_1(&self) -> usize {
        let mut sum = 0;
        for (idy, line) in self.floor.field.iter().enumerate() {
            for (idx, tile) in line.iter().enumerate() {
                if *tile == 'O' {
                    sum += (idy * 100) + idx;
                }
            }
        }
        sum
    }
    fn do_the_robot_part_1(&mut self) {
        self.find_robot();
        for direction in self.robot.move_list.clone() {
            let current_tile = self.robot.position;
            match self.get_moves(&direction, &current_tile, Vec::new()) {
                Some(mut moves) => {
                    self.process_moves(&mut moves);
                }
                None => continue,
            }
        }
    }
    fn find_robot(&mut self) {
        'outer: for (idy, line) in self.floor.field.iter().enumerate() {
            for (idx, c) in line.iter().enumerate() {
                if *c == '@' {
                    self.robot.position = Point::from((idx, idy));
                    break 'outer;
                }
            }
        }
    }
    fn get_moves(&self, dir: &char, current_tile: &Point, moves: Vec<Point>) -> Option<Vec<Point>> {
        let mut moves = moves;
        let next = self.get_next_tile(&dir, current_tile);
        match self.floor.get_point(&next).unwrap() {
            '#' => return None,
            'O' => {
                moves.push(next);
                match self.get_moves(&dir, &next, moves) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            '.' => {
                moves.push(next);
                return Some(moves);
            }
            _ => panic!("Thats nowhere to be found in this room"),
        }
    }
    fn get_next_tile(&self, dir: &char, current_tile: &Point) -> Point {
        match *dir {
            '^' => *current_tile + Point::NORTH,
            'v' => *current_tile + Point::SOUTH,
            '>' => *current_tile + Point::EAST,
            '<' => *current_tile + Point::WEST,
            _ => panic!("not a valid character"),
        }
    }
    fn process_moves(&mut self, moves: &mut Vec<Point>) {
        loop {
            let to = moves.pop().unwrap();
            match moves.last() {
                Some(from) => self.make_move(to, *from),
                None => {
                    self.make_move(to, self.robot.position);
                    self.robot.position = to;
                    break;
                }
            };
        }
    }
    fn make_move(&mut self, to: Point, from: Point) {
        let temp = self.floor.get_point(&to).unwrap();
        self.floor
            .set_point(&to, &self.floor.get_point(&from).unwrap())
            .unwrap();
        self.floor.set_point(&from, &temp);
    }
}
impl WareHouse {
    fn sum_gps_part_2(&self) -> usize {
        let mut sum = 0;
        for (idy, line) in self.floor.field.iter().enumerate() {
            for (idx, tile) in line.iter().enumerate() {
                if *tile == '[' {
                    sum += (idy * 100) + idx;
                }
            }
        }
        sum
    }

    fn expand_floor(&mut self) {
        let mut new_floor = Vec::with_capacity(self.floor.height as usize);
        for line in &self.floor.field {
            let mut new_line = Vec::with_capacity((self.floor.width * 2) as usize);
            for tile in line {
                match tile {
                    '#' => {
                        new_line.push('#');
                        new_line.push('#');
                    }
                    'O' => {
                        new_line.push('[');
                        new_line.push(']');
                    }
                    '.' => {
                        new_line.push('.');
                        new_line.push('.');
                    }
                    '@' => {
                        new_line.push('@');
                        new_line.push('.');
                    }
                    _ => panic!("invalid match in floor expander: {}", tile),
                }
            }
            new_floor.push(new_line);
        }
        self.floor.field = new_floor;
        self.floor.height = self.floor.field.len() as isize;
        self.floor.width = self.floor.field[0].len() as isize;
    }

    fn do_the_robot_part_2(&mut self) {
        self.find_robot();
        for direction in self.robot.move_list.clone() {
            let current_tile = self.robot.position;
            match direction {
                '^' | 'v' => {
                    match self.get_vertical_moves(
                        &direction,
                        &current_tile,
                        VecDeque::new(),
                        VecDeque::new(),
                    ) {
                        Some(mut moves) => {
                            self.process_vertical_moves(&mut moves, &direction);
                        }
                        None => continue,
                    };
                }
                '>' | '<' => match self.get_horizontal_moves(&direction, &current_tile, Vec::new())
                {
                    Some(mut moves) => {
                        self.process_horizontal_moves(&mut moves);
                    }
                    None => continue,
                },
                _ => panic!("do the robot paniced!"),
            }
        }
    }

    fn get_horizontal_moves(
        &self,
        dir: &char,
        current_tile: &Point,
        moves: Vec<Point>,
    ) -> Option<Vec<Point>> {
        let mut moves = moves;
        let next = self.get_next_tile(dir, &current_tile);
        match self.floor.get_point(&next).unwrap() {
            '#' => return None,
            '[' | ']' => {
                moves.push(next);
                match self.get_horizontal_moves(dir, &next, moves) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            '.' => {
                moves.push(next);
                return Some(moves);
            }
            _ => panic!("Thats nowhere to be found in this room"),
        }
    }

    fn get_vertical_moves(
        &self,
        dir: &char,
        current_tile: &Point,
        moves: VecDeque<Point>,
        to_check: VecDeque<Point>,
    ) -> Option<VecDeque<Point>> {
        let mut moves = moves;
        let mut to_check = to_check;
        let next = self.get_next_tile(&dir, &current_tile);
        match self.floor.get_point(&next).unwrap() {
            '#' => return None,
            '[' => {
                moves.push_back(next);
                to_check.push_back(next + Point::EAST);
                match self.get_vertical_moves(&dir, &next, moves, to_check) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            ']' => {
                moves.push_back(next);
                to_check.push_back(next + Point::WEST);
                match self.get_vertical_moves(&dir, &next, moves, to_check) {
                    Some(moves) => return Some(moves),
                    None => return None,
                }
            }
            '.' => {
                match to_check.pop_front() {
                    Some(next) => {
                        moves.push_back(next);
                        match self.get_vertical_moves(&dir, &next, moves, to_check) {
                            Some(moves) => return Some(moves),
                            None => return None,
                        };
                    }
                    None => return Some(moves),
                };
            }
            _ => panic!(
                "get vertical moves really shit the bed.. next tile is: {}",
                self.floor.get_point(&next).unwrap()
            ),
        }
    }

    fn process_vertical_moves(&mut self, moves: &VecDeque<Point>, dir: &char) {
        let mut moves: Vec<Point> = moves.clone().into_iter().collect();
        moves.sort();
        moves.dedup();
        moves.sort_by_key(|y| y.y);
        let mut moves: VecDeque<Point> = moves.into_iter().collect();
        loop {
            match dir {
                '^' => match moves.pop_front() {
                    Some(from) => {
                        let to = from + Point::NORTH;
                        self.make_move(to, from);
                    }
                    None => {
                        let from = self.robot.position;
                        let to = from + Point::NORTH;
                        self.make_move(to, from);
                        self.robot.position = to;
                        break;
                    }
                },
                'v' => match moves.pop_back() {
                    Some(from) => {
                        let to = from + Point::SOUTH;
                        self.make_move(to, from);
                    }
                    None => {
                        let from = self.robot.position;
                        let to = from + Point::SOUTH;
                        self.make_move(to, from);
                        self.robot.position = to;
                        break;
                    }
                },
                _ => panic!("process vertical moves made a fool of itself.."),
            };
        }
    }

    fn process_horizontal_moves(&mut self, moves: &mut Vec<Point>) {
        loop {
            let to = moves.pop().unwrap();
            match moves.last() {
                Some(from) => self.make_move(to, *from),
                None => {
                    self.make_move(to, self.robot.position);
                    self.robot.position = to;
                    break;
                }
            };
        }
    }
}
#[derive(Debug, Clone, Default)]
struct Robot {
    position: Point,
    move_list: VecDeque<char>,
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut warehouse = input.parse();
    // warehouse.floor.print();
    warehouse.do_the_robot_part_1();
    // warehouse.floor.print();

    println!("Part one: {}", warehouse.sum_gps_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut warehouse = input.parse();
    warehouse.expand_floor();
    warehouse.do_the_robot_part_2();

    println!("Part two: {}", warehouse.sum_gps_part_2());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str, _test_data: &str) {
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
