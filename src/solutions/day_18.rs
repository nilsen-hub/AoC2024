use crate::support::field_tools::{Field, Point};
use std::{collections::HashMap, time::Instant, vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct InputData {
    input: String,
    capacity: usize,
}

impl InputData {
    fn parse_part_1(&self) -> Ram {
        let field_line = vec!['.'; self.capacity];
        let memory = vec![field_line; self.capacity];

        let mut bad_sectors = Vec::with_capacity(3450);
        for line in self.input.lines() {
            let split = line.split(',').collect::<Vec<&str>>();
            bad_sectors.push(Point::from((
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            )));
        }
        Ram {
            memory: Field {
                field: memory,
                width: self.capacity as isize,
                height: self.capacity as isize,
            },
            bad_sectors,
        }
    }

    fn parse_part_2(&self) {}
}

#[derive(Debug, Default, Clone)]
struct Ram {
    memory: Field<char>,
    bad_sectors: Vec<Point>,
}

impl Ram {
    fn fuckup_memory(&mut self) {
        for index in 0..1024 as usize {
            self.memory.set_point(&self.bad_sectors[index], &'#');
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    point: Point,
    dist_fr_neigh: usize,
    dist_fr_start: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Default)]
struct Maze {
    field: Field<char>,
    field_graph: HashMap<Point, Vec<Node>>,
    start: Point,
    end: Point,
}

impl Maze {
    fn make_graph(&mut self) {
        let position = self.start;

        let to_explore = self.get_connected_nodes(position);
        self.field_graph.insert(position, to_explore.clone());

        self.node_crawler(to_explore);
    }
    fn node_crawler(&mut self, mut to_explore: Vec<Node>) {
        loop {
            let node = match to_explore.pop() {
                Some(node) => node,
                None => return,
            };
            if self.field_graph.contains_key(&node.point) {
                continue;
            }
            let mut nodes = self.get_connected_nodes(node.point);
            self.field_graph.insert(node.point, nodes.clone());
            to_explore.append(&mut nodes);
        }
    }
    fn get_connected_nodes(&self, start_pos: Point) -> Vec<Node> {
        let directions = [Point::NORTH, Point::SOUTH, Point::EAST, Point::WEST];
        let mut nodes: Vec<Node> = Vec::with_capacity(5);
        for direction in directions {
            let mut current_pos = start_pos;
            let mut steps = 0;

            'outer: loop {
                current_pos = current_pos + direction;
                if self.field.get_point(&current_pos).unwrap() == '#' {
                    break;
                }

                steps += 1;

                if self.field.get_point(&current_pos).unwrap() == 'E'
                    || self.field.get_point(&current_pos).unwrap() == 'S'
                {
                    let node = Node {
                        point: current_pos,
                        dist_fr_neigh: steps,
                        dist_fr_start: usize::MAX,
                        direction: Direction::East,
                    };
                    nodes.push(node);
                    continue;
                }
                for next in directions {
                    let check = current_pos + next;
                    if self.field.get_point(&check).unwrap() == '.' {
                        let node = Node {
                            point: current_pos,
                            dist_fr_neigh: steps,
                            dist_fr_start: usize::MAX,
                            direction: Direction::East,
                        };
                        nodes.push(node);
                        break 'outer;
                    }
                }
            }
        }
        return nodes;
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let ram = input.parse_part_1();

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
        capacity: 71,
    };

    println!("Day eighteen answers:");
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
