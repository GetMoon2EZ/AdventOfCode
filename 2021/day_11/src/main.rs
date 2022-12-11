use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};

const FLASHED_THIS_STEP: u8 = 200;
const ENERGY_TO_FLASH: u8 = 10;

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

#[cfg(debug_assertions)]
fn print_map(map: &Vec<Vec<u8>>) {
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

fn parse_octopuses(filename: &str) -> Vec<Vec<u8>> {
    let mut matrix = Vec::new();

    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            matrix.push(s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
        }
    }
    return matrix;
}

fn flash_and_propagate(matrix: &mut Vec<Vec<u8>>, x: usize, y: usize) -> u64 {
    if matrix[y][x] == FLASHED_THIS_STEP || matrix[y][x] < ENERGY_TO_FLASH {
        return 0;
    }
    matrix[y][x] = FLASHED_THIS_STEP;
    let mut nb_flashes = 1;
    if y != 0 {
        if x != 0 {
            matrix[y - 1][x - 1] += if matrix[y - 1][x - 1] == FLASHED_THIS_STEP { 0 } else { 1 } ;
            nb_flashes += flash_and_propagate(matrix, x - 1, y - 1);
        }
        matrix[y - 1][x] += if matrix[y - 1][x] == FLASHED_THIS_STEP { 0 } else { 1 } ;
        nb_flashes += flash_and_propagate(matrix, x, y - 1);
        if x + 1 != matrix[y].len() {
            matrix[y - 1][x + 1] += if matrix[y - 1][x + 1] == FLASHED_THIS_STEP { 0 } else { 1 } ;
            nb_flashes += flash_and_propagate(matrix, x + 1, y - 1);
        }
    }

    if x != 0 {
        matrix[y][x - 1] += if matrix[y][x - 1] == FLASHED_THIS_STEP { 0 } else { 1 } ;
        nb_flashes += flash_and_propagate(matrix, x - 1, y);
    }
    if x + 1 != matrix[y].len() {
        matrix[y][x + 1] += if matrix[y][x + 1] == FLASHED_THIS_STEP { 0 } else { 1 } ;
        nb_flashes += flash_and_propagate(matrix, x + 1, y);
    }
    if y + 1 != matrix.len() {
        if x != 0 {
            matrix[y + 1][x - 1] += if matrix[y + 1][x - 1] == FLASHED_THIS_STEP { 0 } else { 1 } ;
            nb_flashes += flash_and_propagate(matrix, x - 1, y + 1);
        }
        matrix[y + 1][x] += if matrix[y + 1][x] == FLASHED_THIS_STEP { 0 } else { 1 } ;
        nb_flashes += flash_and_propagate(matrix, x, y + 1);
        if x + 1 != matrix[y].len() {
            matrix[y + 1][x + 1] += if matrix[y + 1][x + 1] == FLASHED_THIS_STEP { 0 } else { 1 } ;
            nb_flashes += flash_and_propagate(matrix, x + 1, y + 1);
        }
    }
    return nb_flashes;
}

fn simulate_step(matrix: &mut Vec<Vec<u8>>) -> u64 {
    // Increase energy levels by 1
    for row in matrix.iter_mut() {
        for octopus in row.iter_mut() {
            *octopus += 1;
        }
    }

    let mut nb_flashes = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            nb_flashes += flash_and_propagate(matrix, x, y);
        }
    }

    // Reset flashed octopuses
    for row in matrix.iter_mut() {
        for octopus in row.iter_mut() {
            if *octopus == FLASHED_THIS_STEP {
                *octopus = 0;
            }
        }
    }

    return nb_flashes;
}

fn simulate_steps(matrix: &mut Vec<Vec<u8>>, n: Option<usize>) -> u64 {
    let mut nb_flashes = 0;
    match n {
        Some(n) => {
            for _ in 0..n {
                nb_flashes += simulate_step(matrix);
            }
        }
        None => {
            let mut i = 0;
            loop {
                i += 1;
                if simulate_step(matrix) == (matrix.len() as u64) * (matrix[0].len() as u64) {
                    return i;
                }
            }
        }
    }
    return nb_flashes;
}

fn solve_problem_1(filename: &str) {
    let mut matrix = parse_octopuses(filename);

    #[cfg(debug_assertions)]
    print_map(&matrix);

    let ans = simulate_steps(&mut matrix, Some(100));
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut matrix = parse_octopuses(filename);

    #[cfg(debug_assertions)]
    print_map(&matrix);

    let ans = simulate_steps(&mut matrix, None);
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
