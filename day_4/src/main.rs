use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Debug, Copy, Clone)]
struct Section {
    start: i32,
    end: i32
}

impl From<[&str; 2]> for Section {
    fn from(arr: [&str; 2]) -> Self {
        let start = match arr[0].parse::<i32>() {
            Ok(v) => { v },
            Err(e) => { panic!("[CONVERSION ERROR] {}", e); }
        };

        let end = match arr[1].parse::<i32>() {
            Ok(v) => { v },
            Err(e) => { panic!("[CONVERSION ERROR] {}", e); }
        };

        Self {
            start,
            end,
        }
    }
}

impl From<&str> for Section {
    fn from(s: &str) -> Self {
        let v = s.split("-").collect::<Vec<&str>>();
        if v.len() != 2 {
            panic!("[CONVERSION ERROR] Cannot convert {} into Section type", s);
        }
        Self::from([v[0], v[1]])
    }
}

impl Section {
    fn contains(&self, other: Section) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps(&self, other: Section) -> bool {
        return (self.start <= other.end && self.end >= other.start) || (other.start <= self.end && other.end >= self.start);
    }
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let assignments: Vec<&str> = s.split(",").collect();
            if assignments.len() != 2 {
                panic!("[INPUT ERROR] Pair does not contain 2 elves: {:?}", assignments);
            }
            let section_1 = Section::from(assignments[0]);
            let section_2 = Section::from(assignments[1]);

            if section_1.contains(section_2) || section_2.contains(section_1) {
                ans += 1;
            }
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let assignments: Vec<&str> = s.split(",").collect();
            if assignments.len() != 2 {
                panic!("[INPUT ERROR] Pair does not contain 2 elves: {:?}", assignments);
            }

            let section_1 = Section::from(assignments[0]);
            let section_2 = Section::from(assignments[1]);

            if section_1.overlaps(section_2) {
                ans += 1;
            }
        }
    }

    println!("Answer: {:?}", ans);
}

fn main() {
    let arg = Arg::parse();

    match arg.challenge_num {
        1 => { solve_problem_1(&arg.filename); },
        2 => { solve_problem_2(&arg.filename); },
        n => { panic!("[ERROR] Incorrect challenge number {}", n); }
    }
}
