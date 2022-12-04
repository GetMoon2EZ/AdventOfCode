use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::collections::HashMap;
use std::cmp::{max, min};

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let coords = s.split(",")
            .map(|value| match value.parse::<i32>() {
                Ok(num) => { num },
                Err(e) => { panic!("{}", e); }
            }).collect::<Vec<i32>>();

        if coords.len() != 2 {
            panic!("[INPUT ERROR] Cannot convert {} to Point type", s);
        }

        Self {
            x: coords[0],
            y: coords[1],
        }
    }
}

fn add_horizontal_line_to_map(map: &mut HashMap<Point, i32>, point_1: Point, point_2: Point) {
    // Horizontal or vertical lines only
    if point_1.x == point_2.x {
        let mut current_y = min(point_1.y, point_2.y);
        while current_y <= max(point_1.y, point_2.y) {
            map.entry(
                Point { x: point_1.x, y: current_y }
            ).and_modify(|counter| *counter += 1).or_insert(1);
            current_y += 1;
        }
    } else if point_1.y == point_2.y {
        let mut current_x = min(point_1.x, point_2.x);
        while current_x <= max(point_1.x, point_2.x) {
            map.entry(
                Point { x: current_x, y: point_1.y }
            ).and_modify(|counter| *counter += 1).or_insert(1);
            current_x += 1;
        }
    }
}

fn add_diagonal_line_to_map(map: &mut HashMap<Point, i32>, point_1: Point, point_2: Point) {
    if point_1.x > point_2.x {
        add_diagonal_line_to_map(map, point_2, point_1);
        return;
    }
    let coefficient = if point_1.x == point_2.x {
        0
    } else {
        (point_2.y - point_1.y) / (point_2.x - point_1.x)
    };

    if coefficient.abs() == 1 {
        let (mut x, mut y, max_x, max_y) = (point_1.x, point_1.y, point_2.x, point_2.y);
        while x <= max_x {
            map.entry(
                Point { x, y }
            ).and_modify(|counter| *counter += 1).or_insert(1);
            x += 1;
            y += coefficient;
        }
    }
}

fn add_line_to_map(map: &mut HashMap<Point, i32>, point_1: Point, point_2: Point) {
    add_horizontal_line_to_map(map, point_1, point_2);
    add_diagonal_line_to_map(map, point_1, point_2);
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;
    let mut map: HashMap<Point, i32> = HashMap::new();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let split: Vec<&str> = s.split(" ").collect();
            if split.len() != 3 {
                panic!("[INPUT ERROR] Invalid input line: {}", s);
            }
            let point_1 = Point::from(split[0]);
            let point_2 = Point::from(split[2]);
            add_horizontal_line_to_map(&mut map, point_1, point_2);
        }
    }

    for value in map.into_values() {
        if value > 1 {
            ans += 1;
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans = 0;
    let mut map: HashMap<Point, i32> = HashMap::new();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let split: Vec<&str> = s.split(" ").collect();
            if split.len() != 3 {
                panic!("[INPUT ERROR] Invalid input line: {}", s);
            }
            let point_1 = Point::from(split[0]);
            let point_2 = Point::from(split[2]);
            add_line_to_map(&mut map, point_1, point_2);
        }
    }

    for value in map.into_values() {
        if value > 1 {
            ans += 1;
        }
    }

    // for y in (0..=9).rev() {
    //     for x in 0..=9 {
    //         print!("{} ", map.entry(Point { x, y }).or_default());
    //     }
    //     print!("\n");
    // }


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
