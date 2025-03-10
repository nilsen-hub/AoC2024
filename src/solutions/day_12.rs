use crate::support::field_tools::{Field, Point};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
    time::Instant,
};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_1(&self) -> Farm {
        let field = Field::from_str(&self.input).unwrap();
        let mut farm = Farm::default();
        farm.field = field;
        farm.directions = [Point::NORTH, Point::SOUTH, Point::EAST, Point::WEST];
        farm
    }
}

#[derive(Debug, Clone, Default)]
struct Farm {
    regions: Vec<Region>,
    field: Field,
    mapped: HashSet<Point>,
    directions: [Point; 4],
}

impl Farm {
    fn solve_part_1(&mut self) -> u32 {
        let mut acc = 0;
        self.walker();
        for region in &self.regions {
            acc += region.area * region.perimeter;
        }
        acc
    }
    fn solve_part_2(&mut self) -> u32 {
        let mut acc = 0;
        self.walker_2();
        for region in &self.regions {
            let side_count = region.side_counter();
            acc += region.area * side_count as u32;
        }
        acc
    }

    fn walker(&mut self) {
        for (idy, line) in self.field.field.iter().enumerate() {
            for (idx, _plot) in line.iter().enumerate() {
                let point = Point::from((idx, idy));
                if self.mapped.contains(&point) {
                    continue;
                }
                let region = self.map_region(point);
                self.regions.push(region.clone());
                self.mapped.extend(region.points);
            }
        }
    }

    fn walker_2(&mut self) {
        for (idy, line) in self.field.field.iter().enumerate() {
            for (idx, _plot) in line.iter().enumerate() {
                let point = Point::from((idx, idy));
                if self.mapped.contains(&point) {
                    continue;
                }
                let region = self.map_region_2(point);
                self.regions.push(region.clone());
                self.mapped.extend(region.points);
            }
        }
    }

    fn map_region_2(&self, point: Point) -> Region {
        // set up region content variables
        let plant = self.field.get_point(&point).unwrap();
        let mut point = point;
        let mut plots: Vec<Plot> = Vec::new();
        let mut checked_points: HashSet<Point> = HashSet::new();
        let mut to_visit: VecDeque<Point> = VecDeque::new();

        // make loop to build and register all connected plots of current plant
        'outer: loop {
            //first build plot struct
            let mut plot = self.get_plot(point);

            for (index, direction) in self.directions.iter().enumerate() {
                let next = point + *direction;
                match self.field.get_point(&next) {
                    Some(target) => {
                        if target == plant {
                            to_visit.push_back(next);
                        } else {
                            plot.fence_count += 1;
                            match index {
                                0 => plot.fences[index] = true,
                                1 => plot.fences[index] = true,
                                2 => plot.fences[index] = true,
                                3 => plot.fences[index] = true,
                                _ => panic!("Something in map_region really messed up"),
                            }
                        }
                    }
                    None => {
                        plot.fence_count += 1;
                        match index {
                            0 => plot.fences[index] = true,
                            1 => plot.fences[index] = true,
                            2 => plot.fences[index] = true,
                            3 => plot.fences[index] = true,
                            _ => panic!("Something in map_region really messed up"),
                        }
                    }
                }
            }
            checked_points.insert(point);
            plots.push(plot);

            if to_visit.len() == 0 {
                break;
            }
            loop {
                point = to_visit.pop_front().unwrap();
                if !checked_points.contains(&point) {
                    break;
                }
                if to_visit.len() == 0 {
                    break 'outer;
                }
            }
        }

        let area = &plots.len();
        let region = Region {
            plots,
            area: *area as u32,
            points: checked_points,
            perimeter: 0,
        };
        region
    }
    fn get_plot(&self, point: Point) -> Plot {
        let plot = Plot {
            point,
            fence_count: 0,
            fences: vec![false; 4],
        };
        plot
    }

    fn map_region(&self, point: Point) -> Region {
        let plant = self.field.get_point(&point).unwrap();
        let mut perimeter = 0;
        let mut point = point;
        let mut checked_points = HashSet::new();
        let mut to_visit = VecDeque::with_capacity(200);

        'outer: loop {
            for direction in self.directions {
                let next = point + direction;
                match self.field.get_point(&next) {
                    Some(target) => {
                        if target == plant {
                            to_visit.push_back(next);
                        } else {
                            perimeter += 1;
                        }
                    }
                    None => {
                        perimeter += 1;
                        continue;
                    }
                }
            }

            checked_points.insert(point);

            loop {
                match to_visit.pop_front() {
                    Some(value) => point = value,
                    None => break 'outer,
                }
                if !checked_points.contains(&point) {
                    break;
                }
            }
        }
        Region {
            area: checked_points.len() as u32,
            perimeter,
            points: checked_points,
            plots: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Region {
    area: u32,
    perimeter: u32,
    points: HashSet<Point>,
    plots: Vec<Plot>,
}

impl Region {
    fn side_counter(&self) -> usize {
        let mut sides: usize = 0;
        let mut fenced_plots: Vec<Plot> = Vec::with_capacity(100);
        let plots = self.plots.clone();
        for plot in plots {
            if plot.fence_count > 0 {
                fenced_plots.push(plot);
            }
        }
        let mut direction_counter: usize = 0;
        loop {
            sides += self.get_sides(&fenced_plots, direction_counter);
            if direction_counter == 3 {
                break;
            }
            direction_counter += 1;
        }
        sides
    }
    fn get_sides(&self, plots: &Vec<Plot>, direction: usize) -> usize {
        // directions by index: North, South, East, West
        let plots = plots.clone();
        let mut directed_plots: Vec<Plot> = Vec::with_capacity(50);
        let mut side_counter: usize = 0;
        let mut plot_map: HashMap<isize, Vec<Plot>> = HashMap::new();

        for plot in plots {
            if plot.fences[direction] {
                directed_plots.push(plot);
            }
        }

        if direction == 0 || direction == 1 {
            // check north and south fences
            for plot in directed_plots {
                plot_map
                    .entry(plot.point.y)
                    .and_modify(|vector| vector.push(plot.clone()))
                    .or_insert(vec![plot]);
            }
            for (_key, mut plot_vector) in plot_map {
                plot_vector.sort_by_key(|plot| plot.point.x);
                for (index, plot) in plot_vector.iter().enumerate() {
                    if index == 0 {
                        side_counter += 1;
                        continue;
                    }
                    if plot.point.x == plot_vector[index - 1].point.x + 1 {
                        continue;
                    }
                    side_counter += 1;
                }
            }
        } else {
            // check east and west fences
            for plot in directed_plots {
                plot_map
                    .entry(plot.point.x)
                    .and_modify(|vector| vector.push(plot.clone()))
                    .or_insert(vec![plot]);
            }
            for (_key, mut plot_vector) in plot_map {
                plot_vector.sort_by_key(|plot| plot.point.y);
                for (index, plot) in plot_vector.iter().enumerate() {
                    if index == 0 {
                        side_counter += 1;
                        continue;
                    }
                    if plot.point.y == plot_vector[index - 1].point.y + 1 {
                        continue;
                    }
                    side_counter += 1;
                }
            }
        }
        side_counter
    }
}

#[derive(Debug, Clone, Default)]
struct Plot {
    point: Point,
    fence_count: u8,
    fences: Vec<bool>, // by index: Nort, South, East, West
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut farm = input.parse_part_1();

    println!("Part one: {}", farm.solve_part_1());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut farm = input.parse_part_1();

    println!("Part two: {}", farm.solve_part_2());
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

pub fn solution(data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day twelve answers:");
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
