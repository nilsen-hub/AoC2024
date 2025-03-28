use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse(&self) -> Towels {
        let mut towels = Towels::default();
        for (index, line) in self.input.lines().into_iter().enumerate() {
            match index {
                0 => {
                    let patterns = line
                        .split_whitespace()
                        .collect::<String>()
                        .split(',')
                        .map(|s| s.chars().collect::<Vec<char>>())
                        .collect::<Vec<Vec<char>>>();

                    towels.patterns = Graph::from(patterns);
                }
                1 => continue,
                _ => towels.designs.push(line.chars().collect()),
            }
        }
        towels
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Graph {
    edges: Vec<Graph>, // [w, u, b, r, g, x];
    status: bool,
}

impl Graph {
    fn from(data: Vec<Vec<char>>) -> Graph {
        let mut node = Graph {
            edges: vec![Graph::default(); 6],
            status: true,
        };

        for d in data {
            node.insert_data(make_vec_usize(&d));
        }

        node
    }

    fn insert_data(&mut self, data: Vec<usize>) {
        let mut next = &mut self.edges;

        for d in data {
            next[d].status = true;
            if !Graph::node_exists(&next[d]) {
                next[d].edges = vec![Graph::default(); 6];
            }
            next = &mut next[d].edges;
        }

        next[5].status = true;
    }

    fn node_exists(node: &Graph) -> bool {
        match node.edges.get(5) {
            Some(_n) => return true,
            None => return false,
        }
    }

    // debug / test- function
    //fn print_data(
    //    &self,
    //    current: &Graph,
    //    to_print: Vec<char>,
    //    //collector: &mut Vec<Vec<char>>,
    //) {
    //    // debug: -> Vec<Vec<char>>
    //    for (index, node) in current.edges.iter().enumerate() {
    //        if node.status {
    //            let mut to_print = to_print.clone();
    //            if index != 5 {
    //                to_print.push(convert_index(&index));
    //            }
    //
    //            match node.edges.get(5) {
    //                Some(n) => {
    //                    if n.status {
    //                        //collector.push(to_print.clone());
    //                        println!("{:?}", &to_print);
    //                    }
    //                }
    //                None => (),
    //            }
    //
    //            self.print_data(&node, to_print); // , to_print, collector
    //        }
    //    }
    //
    //    //collector.clone()
    //}
}

#[derive(Debug, Default, Clone)]
struct Towels {
    // for dubug:
    //debug_set: HashSet<Vec<char>>,
    patterns: Graph,
    designs: Vec<Vec<char>>,
}

impl Towels {
    fn solve_part_1(&self) -> usize {
        let mut acc = 0;
        for design in &self.designs {
            if self.towel_finder(&make_vec_usize(&design), &self.patterns.edges) {
                acc += 1;
            }
        }
        acc
    }

    fn towel_finder(&self, target: &Vec<usize>, graph: &Vec<Graph>) -> bool {
        let mut index = 0;
        let mut graph_history: Vec<usize> = Vec::with_capacity(100);
        let mut current_graph = graph;
        let mut graph_history_seen: Vec<usize> = Vec::with_capacity(80);

        loop {
            if index == target.len() {
                if current_graph[5].status {
                    return true;
                }
                match graph_history.pop() {
                    Some(v) => {
                        index = v;
                        current_graph = &self.patterns.edges;
                    }
                    None => return false,
                }
            }

            if current_graph[target[index]].status {
                current_graph = &current_graph[target[index]].edges;
                index += 1;
                if current_graph[5].status {
                    if !graph_history_seen.contains(&index) {
                        graph_history.push(index);
                        graph_history_seen.push(index);
                    }
                }
                continue;
            }

            if !current_graph[5].status {
                match graph_history.pop() {
                    Some(v) => {
                        index = v;
                        current_graph = &self.patterns.edges;
                        continue;
                    }
                    None => return false,
                }
            } else {
                current_graph = &self.patterns.edges;
            }
        }
    }

    fn towel_finder_complete(&self, target: &Vec<usize>) -> u32 {
        // the idea here ties my brain into a knot, but it should be simple enough
        // I take the target, and send it through the towel checker.
        // If its good, I add one to the acc.
        // I then descend one level into the graph, remove the now covered value
        // from the target.
        // Then I check the next one. first I check which way the original went.
        // If the target[0] box in the graph is false, I know it had to jump to
        // the top, so I will do this aswell. If the target[0] box is true, I check if
        // its valid, if it is, I know that this was the path taken. And then I check if
        // jumping up is true, if it is, I add this graph and target to the to check list.
        // Summed up: If no fork, contiue down, if fork, check validity of looping back. If
        // valid, add to to check list, you now have a new path to figure out.
        // I think this will result in an accurate count of the amount of paths avaliable to
        // target.
        if !self.towel_finder(target, &self.patterns.edges) {
            return 0;
        }
        let mut acc = 1;
        use ValidPaths as P;
        let mut graph = self.patterns.edges[target[0]].clone();
        let mut target: Vec<usize> = target.clone().drain(1..).collect();
        let mut forks: VecDeque<Vec<usize>> = VecDeque::with_capacity(100);
        let mut checked_forks: HashSet<ForkRecord> = HashSet::new();

        loop {
            if target.is_empty() || graph.edges.is_empty() {
                match forks.pop_back() {
                    Some(r) => {
                        //println!("popped fork: {:?}", r);
                        if r.len() == 0 {
                            continue;
                        }
                        graph = self.patterns.clone();
                        target = r;
                    }
                    None => return acc,
                }
            }
            match self.fork_detector(&graph.edges, &target) {
                P::Both => {
                    //println!("fork detected! target len: {}", target.len());
                    acc += 1;
                    //println!("acc: {}", acc);
                    //println!("target: {:?}", target);
                    let record = ForkRecord {
                        graph: graph.clone(),
                        target: target.clone(),
                    };
                    if !checked_forks.contains(&record) {
                        checked_forks.insert(record);
                        forks.push_back(target.clone());
                    }

                    graph = graph.edges[target[0]].clone();
                    target = target.drain(1..).collect();
                }
                P::Retur => {
                    //println!("no fork here (return) target len: {}", target.len());
                    graph = self.patterns.edges[target[0]].clone();
                    target = target.drain(1..).collect();
                    if target.len() == 0 {
                        continue;
                    }
                }
                P::Ahead => {
                    //println!("no fork here (ahead) target len: {}", target.len());
                    graph = graph.edges[target[0]].clone();
                    target = target.drain(1..).collect();
                    if target.len() == 0 {
                        continue;
                    }
                }
            }
        }
    }

    // The fork detector checks if both ahead and return path results in a valid towel.
    // if only one path is true
    fn fork_detector(&self, graph: &Vec<Graph>, target: &Vec<usize>) -> ValidPaths {
        let mut ahead = false;
        let mut retur = false;
        //println!("target: {:?}", target);
        if graph[5].status {
            retur = self.towel_finder(&target, &self.patterns.edges);
        }

        if graph[target[0]].status {
            ahead = self.towel_finder(&target, &graph[target[0]].edges);
        }

        if retur && ahead {
            return ValidPaths::Both;
        }
        if retur {
            return ValidPaths::Retur;
        }

        return ValidPaths::Ahead;
    }

    fn solve_part_2(&self) -> u32 {
        let mut acc = 0;
        for (index, design) in self.designs.iter().enumerate() {
            //println!("Checking design {}", index + 1);
            acc += self.towel_finder_complete(&make_vec_usize(design));
        }
        acc
    }
}

enum ValidPaths {
    Ahead,
    Retur,
    Both,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct ForkRecord {
    graph: Graph,
    target: Vec<usize>,
}

fn make_vec_usize(vec: &Vec<char>) -> Vec<usize> {
    let mut output = Vec::new();
    for c in vec {
        output.push(convert_color(c));
    }
    output
}

fn convert_color(c: &char) -> usize {
    match c {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        _ => panic!("expected different value"),
    }
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let towels = input.parse();

    println!("Part one: {}", towels.solve_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let towels = input.parse();

    println!("Part two: {}", towels.solve_part_2());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str, test_data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    let test = InputData {
        input: test_data.to_string(),
    };

    println!("Day nineteen answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&test);
    println!("");
}

// testing functions below

//fn convert_index(i: &usize) -> char {
//    match i {
//        0 => 'w',
//        1 => 'u',
//        2 => 'b',
//        3 => 'r',
//        4 => 'g',
//        _ => panic!("expected different value"),
//    }
//}

//fn test_graph_completeness(graph_output: Vec<Vec<char>>, mut graph_input: HashSet<Vec<char>>) {
//    println!(
//        "collector len, input len: {}, {}",
//        graph_output.len(),
//        graph_input.len()
//    );
//    for line in graph_output {
//        if graph_input.remove(&line) {
//            println!("present in both sets: {:?}", line);
//        } else {
//            println!("phantom vector: {:?}", line);
//        }
//    }
//    println!("Should be zero: {}", graph_input.len());
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn parser_works() {}
//}
