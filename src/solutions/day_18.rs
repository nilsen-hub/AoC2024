use crate::support::field_tools::{Field, Point};
use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
    usize, vec,
};

pub fn solution(data: &str, _test_data: &str) {
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

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut ram = input.parse();

    println!("Part one: {}", ram.solve_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut ram = input.parse();
    let point = ram.solve_part_2();

    println!("Part two: {},{}", point.x, point.y);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

#[derive(Debug, Clone)]
struct InputData {
    input: String,
    capacity: usize,
}

impl InputData {
    fn parse(&self) -> Ram {
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
            maze: Maze::default(),
            directions: [Point::NORTH, Point::SOUTH, Point::EAST, Point::WEST],
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Ram {
    memory: Field<char>,
    bad_sectors: Vec<Point>,
    maze: Maze,
    directions: [Point; 4],
}

impl Ram {
    fn solve_part_1(&mut self) -> usize {
        self.fuckup_memory(1024);
        self.build_maze();
        self.maze.dijkstra()
    }

    fn solve_part_2(&mut self) -> Point {
        let mut lower = 1024;
        let mut upper = self.bad_sectors.len();

        loop {
            let check = self.bin_search_number(lower, upper);
            self.fuckup_memory(check);
            self.build_maze();

            if self.maze.field_graph.contains_key(&self.maze.end) {
                lower = check;
            } else {
                upper = check;
            }

            if upper - lower <= 1 {
                return self.bad_sectors[lower];
            }

            self.reset_memory();
        }
    }
    fn reset_memory(&mut self) {
        let capacity = self.memory.height as usize;
        let field_line = vec!['.'; capacity];
        self.memory.field = vec![field_line; capacity];
    }
    fn bin_search_number(&self, lower: usize, upper: usize) -> usize {
        ((upper - lower) / 2) + lower
    }
    fn fuckup_memory(&mut self, amnt: usize) {
        for index in 0..amnt as usize {
            self.memory.set_point(&self.bad_sectors[index], &'#');
        }
    }
    fn build_maze(&mut self) {
        self.maze.field = self.memory.clone();
        self.maze.start = Point::from((0, 0));
        self.maze.end = Point::from((70, 70));
        self.maze.directions = self.directions;
        self.maze.field_graph.clear();
        self.maze.build_graph();
    }

    //fn test_maze(&self) {
    //    let graph = &self.maze.field_graph;
    //    let mut field = self.maze.field.clone();
    //    for (p, _v) in graph {
    //        let point = field.get_point(&p).unwrap();
    //        if point != '.' {
    //            println!("THIS POINT SUCKS: {:?}", p);
    //        }
    //        field.set_point(&p, &'O').unwrap();
    //    }
    //    field.print();
    //}
}

#[derive(Debug, Clone, Default)]
struct Node {
    point: Point,
    dist_neigh: usize,
    dist_start: usize,
}

#[derive(Debug, Clone, Default)]
struct Maze {
    field: Field<char>,
    field_graph: HashMap<Point, Vec<Node>>,
    start: Point,
    end: Point,
    directions: [Point; 4],
}

impl Maze {
    fn build_graph(&mut self) {
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
        let mut nodes = Vec::with_capacity(4);
        for dir in self.directions {
            let current_pos = start_pos + dir;
            let check = match self.field.get_point(&current_pos) {
                Some(tile) => tile,
                None => continue,
            };
            if check == '.' {
                nodes.push(Node {
                    point: current_pos,
                    dist_neigh: 1,
                    dist_start: usize::MAX,
                });
            }
        }

        nodes
    }

    fn dijkstra(&mut self) -> usize {
        let mut frontier = BTreeMap::new();
        let mut visited: HashMap<Point, Node> = HashMap::new();

        frontier.insert(
            0,
            Vec::from([Node {
                point: self.start,
                dist_neigh: 0,
                dist_start: 0,
            }]),
        );
        loop {
            let current_nodes = frontier.pop_first().unwrap().1;
            for node in current_nodes {
                if let Some(n) = visited.get(&node.point) {
                    if node.dist_start >= n.dist_start {
                        continue;
                    }
                }

                let connected_nodes = self.field_graph.get(&node.point).unwrap().clone();

                for mut dest in connected_nodes {
                    dest.dist_start = node.dist_start + dest.dist_neigh;
                    if dest.point == self.end {
                        return dest.dist_start;
                    }

                    frontier
                        .entry(dest.dist_start)
                        .and_modify(|v| v.push(dest.clone()))
                        .or_insert(vec![dest]);
                }

                visited.insert(node.point, node);
            }
        }
    }
}
