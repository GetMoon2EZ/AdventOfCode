use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::cmp::max;

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Debug, Copy, Clone)]
struct Tree {
    visible: bool,
    height: i32,
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn parse_input(filename: &str) -> Vec<Vec<Tree>> {
    let mut matrix = Vec::new();
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            matrix.push(
                s.chars().map(|c| Tree { visible: false, height: c.to_digit(10).unwrap() as i32 } ).collect::<Vec<Tree>>()
            );
        }
    }
    return matrix;
}

fn solve_problem_1(filename: &str) {
    let mut matrix = parse_input(filename);

    let mut ans = 0;
    let mut max_left = Vec::new();
    let mut max_right = Vec::new();
    let mut max_up = Vec::new();
    let mut max_down = Vec::new();

    for i_row in 0..matrix.len() {
        for i_col in 0..matrix[i_row].len() {
            if i_row == 0 {
                max_up.push(-1);
                max_down.push(-1);
            }
            if i_col == 0 {
                max_left.push(-1);
                max_right.push(-1);
            }
            let back_col = matrix[i_row].len() - i_col - 1;
            let back_row = matrix.len() - i_row - 1;
            if matrix[i_row][i_col].height > max_left[i_row] || matrix[i_row][i_col].height > max_up[i_col] {
                if !matrix[i_row][i_col].visible {
                    ans += 1;
                    matrix[i_row][i_col].visible = true;
                }
                max_left[i_row] = max(max_left[i_row], matrix[i_row][i_col].height);
                max_up[i_col] = max(max_up[i_col], matrix[i_row][i_col].height);
            }
            if matrix[i_row][back_col].height > max_right[i_row] {
                if !matrix[i_row][back_col].visible {
                    ans += 1;
                    matrix[i_row][back_col].visible = true;
                }
                max_right[i_row] = matrix[i_row][back_col].height;
            }
            if matrix[back_row][i_col].height > max_down[i_col] {
                if !matrix[back_row][i_col].visible {
                    ans += 1;
                    matrix[back_row][i_col].visible = true;
                }
                max_down[i_col] = matrix[back_row][i_col].height;
            }
        }
    }
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let matrix = parse_input(filename);
    let mut ans = 0;

    for i_row in 1..matrix.len() - 1 {
        for i_col in 1..matrix[i_row].len() - 1 {
            let mut scenic_score = 1;
            // Count up
            for i in (0..i_row).rev() {
                if matrix[i][i_col].height >= matrix[i_row][i_col].height || i == 0 {
                    scenic_score = scenic_score * (i_row - i);
                    break;
                }
            }
            // Count down
            for i in (i_row+1)..matrix.len() {
                if matrix[i][i_col].height >= matrix[i_row][i_col].height || i == matrix.len() - 1 {
                    scenic_score = scenic_score * (i - i_row);
                    break;
                }
            }
            // Count left
            for i in (0..i_col).rev() {
                if matrix[i_row][i].height >= matrix[i_row][i_col].height || i == 0 {
                    scenic_score = scenic_score * (i_col - i);
                    break;
                }
            }
            // Count right
            for i in (i_col+1)..matrix[i_row].len() {
                if matrix[i_row][i].height >= matrix[i_row][i_col].height || i == matrix[i_row].len() - 1 {
                    scenic_score = scenic_score * (i - i_col);
                    break;
                }
            }
            ans = max(ans, scenic_score);
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
