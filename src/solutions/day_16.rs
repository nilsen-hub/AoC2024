use std::{
    collections::{BTreeMap, HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use crate::support::field_tools::{Field, Point};

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
}

impl InputData {
    fn parse(&self) -> Solver {
        let field = Field::from_str(&self.input).unwrap();

        let mut end = Point::default();
        for (idx, c) in field.field[1].iter().enumerate() {
            if *c == 'E' {
                end = Point::from((idx, 1));
            }
        }

        let start = Point::from((end.y, end.x));

        Solver {
            maze: Maze {
                field,
                field_graph: HashMap::new(),
                start,
                end,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    point: Point,
    dist_fr_neigh: usize,
    dist_fr_start: usize,
    direction: Direction,
    path: HashSet<Point>,
}

#[derive(Debug, Clone)]
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
                        path: HashSet::new(),
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
                            path: HashSet::new(),
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

#[derive(Debug, Clone)]
struct Solver {
    maze: Maze,
}
impl Solver {
    fn solve_part_1(&mut self) -> usize {
        self.maze.make_graph();
        let mut frontier: BTreeMap<usize, Vec<Node>> = BTreeMap::new();
        let mut visited: HashMap<Point, Node> = HashMap::new();

        frontier.insert(
            0,
            Vec::from([Node {
                point: self.maze.start,
                dist_fr_neigh: 0,
                dist_fr_start: 0,
                direction: Direction::East,
                path: HashSet::new(),
            }]),
        );

        loop {
            let current_nodes = match frontier.pop_first() {
                Some(vec) => vec.1,
                None => panic!("This should not happen"),
            };
            for node in current_nodes {
                if let Some(nod) = visited.get(&node.point) {
                    if node.dist_fr_start >= nod.dist_fr_start {
                        continue;
                    }
                }

                let connected_nodes = self.maze.field_graph.get(&node.point).unwrap().clone();

                for mut destination in connected_nodes {
                    destination.dist_fr_start = node.dist_fr_start + destination.dist_fr_neigh;
                    destination.direction = self.turn_detector(&node, destination.point);

                    if destination.direction != node.direction {
                        destination.dist_fr_start += 1000;
                    }

                    if destination.point == self.maze.end {
                        return destination.dist_fr_start;
                    }

                    frontier
                        .entry(destination.dist_fr_start)
                        .and_modify(|vec| vec.push(destination.clone()))
                        .or_insert(Vec::from([destination]));
                }
                visited.insert(node.point, node);
            }
        }
    }

    fn solve_part_2(&mut self) -> usize {
        self.maze.make_graph();

        let mut frontier: HashMap<Point, Vec<Node>> = HashMap::new();
        let mut finishers: BTreeMap<usize, Vec<Node>> = BTreeMap::new();

        frontier.insert(
            self.maze.start,
            Vec::from([Node {
                point: self.maze.start,
                dist_fr_neigh: 0,
                dist_fr_start: 0,
                direction: Direction::East,
                path: HashSet::new(),
            }]),
        );

        loop {
            let mut current_nodes: Vec<Node> = Vec::new();
            if frontier.len() == 0 {
                break;
            }
            for node_vector in frontier.clone() {
                let mut to_insert = node_vector.1;
                to_insert.sort_by_key(|node| node.dist_fr_start);
                let comp = to_insert[0].dist_fr_start;
                for node in to_insert {
                    if comp.abs_diff(node.dist_fr_start) <= 1000 {
                        current_nodes.push(node);
                    } else {
                        break;
                    }
                }
            }
            //println!("current nodes length: {}", current_nodes.len());
            frontier.clear();

            for mut node in current_nodes {
                if node.dist_fr_start > 75000 {
                    continue;
                }

                node.path.insert(node.point);
                let connected_nodes = self.maze.field_graph.get(&node.point).unwrap().clone();

                for mut destination in connected_nodes {
                    if node.path.contains(&destination.point) {
                        continue;
                    }

                    destination.path = node.path.clone();
                    destination.dist_fr_start = node.dist_fr_start + destination.dist_fr_neigh;
                    destination.direction = self.turn_detector(&node, destination.point);

                    if destination.direction != node.direction {
                        destination.dist_fr_start += 1000;
                    }

                    if destination.point == self.maze.end {
                        destination.path.insert(destination.point);
                        finishers
                            .entry(destination.dist_fr_start)
                            .and_modify(|vec| vec.push(destination.clone()))
                            .or_insert(Vec::from([destination]));
                        break;
                    }

                    frontier
                        .entry(destination.point)
                        .and_modify(|vec| vec.push(destination.clone()))
                        .or_insert(Vec::from([destination]));
                }
            }
        }

        let to_check = finishers.pop_first().unwrap().1;
        let mut printable: HashSet<Point> = HashSet::new();

        for node in to_check {
            printable.extend(node.path);
        }
        //self.maze.point_printer(&printable);

        return printable.len();
    }
    fn turn_detector(&self, node: &Node, next_pos: Point) -> Direction {
        let dir_indicator = node.point - next_pos;
        use Direction as D;
        match node.direction {
            D::North | D::South => {
                if dir_indicator.x == 0 {
                    return node.direction;
                }
                if dir_indicator.x.is_negative() {
                    return D::East;
                } else {
                    return D::West;
                }
            }
            D::East | D::West => {
                if dir_indicator.y == 0 {
                    return node.direction;
                }
                if dir_indicator.y.is_negative() {
                    return D::South;
                } else {
                    return D::North;
                }
            }
        }
    }
}
fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut solver = input.parse();

    println!("Part one: {}", solver.solve_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut solver = input.parse();

    println!("Part two: {}", solver.solve_part_2());
    println!("Runtime (micros): {}", now.elapsed().as_secs_f32());
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day sixteen answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
