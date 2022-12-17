use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::collections::HashMap;
use std::cmp::{min, max};

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

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl From<&str> for Coordinates {
    fn from(s: &str) -> Self {
        let split = s.split(", ").map(|s| String::from(s)).collect::<Vec<String>>();
        if split.len() != 2 {
            panic!("[ERROR] Cannot convert {} to Coordinates", s);
        }
        let x = split[0].strip_prefix("x=").unwrap().parse::<i32>().unwrap();
        let y = split[1].strip_prefix("y=").unwrap().parse::<i32>().unwrap();
        Self { x, y }
    }
}

impl Coordinates {
    fn get_manhattan_distance_to(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn get_tunning_frequency(&self) -> u64 {
        (self.x as u64) * 4000000 + (self.y as u64)
    }
}

fn parse_input(filename: &str) -> Vec<(Coordinates, Coordinates)> {
    let mut ret = Vec::new();
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let s = s.strip_prefix("Sensor at ").unwrap();
            let split = s.split(": ").collect::<Vec<&str>>();
            let sensor_pos = Coordinates::from(split[0]);
            let beacon_pos = Coordinates::from(split[1].strip_prefix("closest beacon is at ").unwrap());
            ret.push((sensor_pos, beacon_pos));
        }
    }
    return ret;
}

fn get_row_coverage(positions: &Vec<(Coordinates, Coordinates)>, row: i32) -> usize {
    let mut map: HashMap<i32, bool> = HashMap::new();
    for (sensor_pos, beacon_pos) in positions.iter() {
        let distance = sensor_pos.get_manhattan_distance_to(*beacon_pos);
        if sensor_pos.y >= row {
            if sensor_pos.y - distance <= row {
                let spare_distance = (row - (sensor_pos.y - distance)).abs();
                for current_x in (sensor_pos.x - spare_distance)..=(sensor_pos.x + spare_distance) {
                    map.entry(current_x).or_insert(true);
                }
            }
        } else {
            if sensor_pos.y + distance >= row {
                let spare_distance = (row - (sensor_pos.y + distance)).abs();
                for current_x in (sensor_pos.x - spare_distance)..=(sensor_pos.x + spare_distance) {
                    map.entry(current_x).or_insert(true);
                }
            }
        }
    }

    if map.len() > 0 {
        return map.len() - 1;
    }
    return 0;
}

fn find_distress_beacon(positions: &Vec<(Coordinates, Coordinates)>, extremum: Coordinates) -> Coordinates {
    let mut intervals: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for (sensor_pos, beacon_pos) in positions.iter() {
        let distance = sensor_pos.get_manhattan_distance_to(*beacon_pos); // 7
        let min_y = max(sensor_pos.y - distance, 0); // 18 - 7 = 11
        let max_y = min(sensor_pos.y + distance, extremum.y); // 18 + 7 = 25 -> 20
        for current_y in min_y..=max_y { // 19
            let spare_distance = if sensor_pos.y <= current_y {
                (current_y - (sensor_pos.y + distance)).abs()
            } else {
                (current_y - (sensor_pos.y - distance)).abs()
            };
            let min_x = max(sensor_pos.x - spare_distance, 0); // 0
            let max_x = min(sensor_pos.x + spare_distance, extremum.x); // 4
            intervals.entry(current_y)
                .and_modify(|v| v.push((min_x, max_x)))
                .or_insert(vec!((min_x, max_x)));
        }
    }

    // println!("[DEBUG] {:#?}", intervals);

    for y in 0..=extremum.y {
        let mut left = 0;
        let mut right = extremum.x;
        loop {
            let prev_left = left;
            let prev_right = right;
            for (min, max) in intervals.get(&y).unwrap().iter() {
                if *min <= left && *max > left {
                    left = *max;
                }
                if *max >= right && *min < right {
                    right = *min;
                }
            }
            if left > right || (prev_left == left && prev_right == right) {
                break;
            }
        }
        if left < right {
            println!("[DEBUG] Found gap at {}, {}", left + 1, y);
            return Coordinates { x: left + 1, y };
        }
    }
    println!("[DEBUG] Did not find gap");
    return extremum;
}

fn solve_problem_1(filename: &str) {
    let positions = parse_input(filename);
    let ans = get_row_coverage(&positions, 2000000);
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let positions = parse_input(filename);
    let distress_beacon = find_distress_beacon(&positions, Coordinates { x: 4000000, y: 4000000 });
    let ans = distress_beacon.get_tunning_frequency();
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
