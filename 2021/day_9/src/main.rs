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

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn parse_input(filename: &str) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();

    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            map.push(s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
        }
    }

    return map;
}

fn get_basin_size(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if map[x][y] == 9 {
        return 0;
    }
    map[x][y] = 9;
    let mut basin_size = 1;
    if x != 0 {
        basin_size += get_basin_size(map, x - 1, y);
    }
    if y != 0 {
        basin_size += get_basin_size(map, x, y - 1);
    }
    if x + 1 != map.len() {
        basin_size += get_basin_size(map, x + 1, y);
    }
    if y + 1 != map[x].len() {
        basin_size += get_basin_size(map, x, y + 1);
    }
    return basin_size;
}

#[cfg(debug_assertions)]
fn print_map(map: &Vec<Vec<u32>>) {
    // Assuming the map is a square
    let mut top = "┏".to_string();
    top.push_str((0..map[0].len()).map(|_| "━").collect::<String>().as_str());
    top.push_str("┓");
    let mut bottom = "┗".to_string();
    bottom.push_str((0..map[0].len()).map(|_| "━").collect::<String>().as_str());
    bottom.push_str("┛");
    println!("{}", top);
    for x in 0..map.len() {
        print!("┃");
        for y in 0..map[x].len() {
            print!("{}", map[x][y]);
        }
        print!("┃\n");
    }
    println!("{}", bottom);
}

fn get_basins(map: &mut Vec<Vec<u32>>) -> Vec<u32> {
    let mut basins: Vec<u32> = Vec::new();
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            let basin_size = get_basin_size(map, x, y);
            if basin_size != 0 {
                #[cfg(debug_assertions)]
                print_map(map);
                basins.push(basin_size)
            }
        }
    }
    return basins;
}

fn get_biggest(array: &Vec<u32>, n: usize) -> Vec<u32> {
    let mut biggest = vec![0; n];
    for val in array.iter() {
        let mut value = *val;
        for max in biggest.iter_mut() {
            if *max < value {
                let tmp = *max;
                *max = value;
                value = tmp;
            }
        }
    }
    return biggest;
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;
    let map = parse_input(filename);
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            // Check neighbors
            if x != 0 {
                if map[x][y] >= map[x - 1][y] {
                    continue;
                }
            }
            if y != 0 {
                if map[x][y] >= map[x][y - 1] {
                    continue;
                }
            }
            if x + 1 != map.len() {
                if map[x][y] >= map[x + 1][y] {
                    continue;
                }
            }
            if y + 1 != map[x].len() {
                if map[x][y] >= map[x][y + 1] {
                    continue;
                }
            }
            ans += map[x][y] + 1;
        }
    }
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut map = parse_input(filename);
    let basins = get_basins(&mut map);
    let ans = get_biggest(&basins, 3).iter().fold(1, |res, a| res * a);
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
