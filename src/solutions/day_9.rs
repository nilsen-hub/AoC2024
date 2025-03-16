use std::{collections::VecDeque, time::Instant};

#[derive(Debug, Clone)]
struct InputData {
    input: String,
}

impl InputData {
    fn parse_part_one(&self) -> HardDrive {
        let (files, gaps) = self.get_files_and_gaps_part_one();
        HardDrive { files, gaps }
    }

    fn parse_part_two(&self) -> HardDrive {
        let (files, gaps) = self.get_files_and_gaps_part_two();
        HardDrive { files, gaps }
    }
    fn get_files_and_gaps_part_one(&self) -> (VecDeque<File>, VecDeque<Gap>) {
        let string: Vec<char> = self.input.chars().collect();
        let mut files: VecDeque<File> = VecDeque::with_capacity(5000);
        let mut gaps: VecDeque<Gap> = VecDeque::with_capacity(5000);
        let bounds = string.len();
        // to deal with zero-case
        files.push_back(build_file(0, string[0]));
        let mut count = 1;
        loop {
            // kinda ugly, but should work
            gaps.push_back(build_gap(string[count]));
            count += 1;
            if count == bounds {
                break;
            }
            files.push_back(build_file(count / 2, string[count]));
            count += 1;
            if count == bounds {
                break;
            }
        }

        (files, gaps)
    }

    fn get_files_and_gaps_part_two(&self) -> (VecDeque<File>, VecDeque<Gap>) {
        let string: Vec<char> = self.input.chars().collect();
        let mut files: VecDeque<File> = VecDeque::with_capacity(5000);
        let mut gaps: VecDeque<Gap> = VecDeque::with_capacity(5000);
        let bounds = string.len();
        // to deal with zero-case
        files.push_back(build_file(0, string[0]));
        let mut count = 1;
        loop {
            // kinda ugly, but should work
            let mut gap = build_gap(string[count]);
            gap.start_index = files.back().unwrap().start_index + files.back().unwrap().size;
            gaps.push_back(gap);
            count += 1;
            if count == bounds {
                break;
            }
            let mut file = build_file(count / 2, string[count]);
            file.start_index = gaps.back().unwrap().start_index + gaps.back().unwrap().size;
            files.push_back(file);
            count += 1;
            if count == bounds {
                break;
            }
        }

        (files, gaps)
    }
}

struct HardDrive {
    files: VecDeque<File>,
    gaps: VecDeque<Gap>,
}

impl HardDrive {
    fn fix_start_indices(&mut self) {
        let mut index: usize = 0;
        let bounds = self.files.len();
        loop {
            if index == 0 {
                index += 1;
                continue;
            }
            let prev = &self.files[index - 1];
            self.files[index].start_index = prev.start_index + prev.size;
            index += 1;
            if index == bounds {
                break;
            }
        }
    }
    fn compress_part_1(&mut self) {
        let mut comp_files: VecDeque<File> = VecDeque::with_capacity(5000);

        // this is where the really clunky gears start turning.
        'outer: loop {
            comp_files.push_back(match self.files.pop_front() {
                Some(file) => file,
                None => break,
            });
            let mut last_file = match self.files.pop_back() {
                Some(file) => file,
                None => break,
            };
            let mut current_gap = match self.gaps.pop_front() {
                Some(gap) => gap,
                None => break,
            };
            loop {
                if last_file.size == current_gap.size {
                    comp_files.push_back(last_file);
                    continue 'outer;
                }
                if last_file.size < current_gap.size {
                    current_gap.size -= last_file.size;
                    comp_files.push_back(last_file);
                    last_file = match self.files.pop_back() {
                        Some(file) => file,
                        None => break 'outer,
                    };
                    continue;
                }
                if last_file.size > current_gap.size {
                    if self.files.len() == 0 {
                        comp_files.push_back(last_file);
                        break 'outer;
                    }
                    let new_file = File {
                        id: last_file.id,
                        size: current_gap.size,
                        start_index: last_file.start_index,
                    };
                    last_file.size -= new_file.size;
                    comp_files.push_back(new_file);
                    comp_files.push_back(match self.files.pop_front() {
                        Some(file) => file,
                        None => {
                            comp_files.push_back(last_file);
                            break 'outer;
                        }
                    });
                    current_gap = match self.gaps.pop_front() {
                        Some(gap) => gap,
                        None => panic!("literally impossible"),
                    };
                }
            }
        }
        self.files = comp_files;
    }

    fn compress_part_2(&mut self) {
        let mut compressed_files: VecDeque<File> = VecDeque::with_capacity(5000);
        let mut files = self.files.clone();
        let mut gaps = self.gaps.clone();
        files.make_contiguous().reverse();
        'outer: for mut file in files {
            let mut gap_index: usize = usize::MAX;
            for (index, gap) in gaps.make_contiguous().iter().enumerate() {
                if file.start_index <= gap.start_index {
                    compressed_files.push_front(file);
                    continue 'outer;
                }
                if file.size <= gap.size {
                    gap_index = index;
                    break;
                }
            }
            if file.size == gaps[gap_index].size {
                file.start_index = gaps[gap_index].start_index;
                compressed_files.push_front(file);
                gaps.remove(gap_index);
            } else {
                file.start_index = gaps[gap_index].start_index;
                gaps[gap_index].start_index += file.size;
                gaps[gap_index].size -= file.size;
                compressed_files.push_front(file);
            }
        }
        self.files = compressed_files;
    }
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    size: usize,
    start_index: usize,
}

impl File {
    fn get_checksum(&self) -> usize {
        let mut acc: usize = 0;
        let size = self.size;
        let mut count = self.start_index;
        while count < self.start_index + size {
            acc += self.id * count;
            count += 1;
        }
        acc
    }
}

#[derive(Debug, Clone, Copy)]
struct Gap {
    size: usize,
    start_index: usize,
}

fn part_1(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let mut hdd = input.parse_part_one();
    hdd.compress_part_1();
    hdd.fix_start_indices();

    for file in hdd.files {
        acc += file.get_checksum();
    }

    println!("Part one: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn part_2(input: &InputData) {
    let now = Instant::now();
    let mut acc: usize = 0;

    let mut hdd = input.parse_part_two();
    hdd.compress_part_2();

    for file in hdd.files {
        acc += file.get_checksum();
    }

    println!("Part two: {}", acc);
    println!("Runtime (micros): {}", now.elapsed().as_micros());
}

fn build_file(id: usize, size: char) -> File {
    let size: usize = size.to_digit(10).unwrap() as usize;
    File {
        id,
        size,
        start_index: 0,
    }
}
fn build_gap(size: char) -> Gap {
    let size: usize = size.to_digit(10).unwrap() as usize;
    let start_index = 0;
    Gap { size, start_index }
}

pub fn solution(data: &str, _test_data: &str) {
    let input = InputData {
        input: data.to_string(),
    };

    println!("Day nine answers:");
    println!("");

    part_1(&input);
    println!("");
    part_2(&input);
    println!("");
}
