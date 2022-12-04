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

fn solve_problem_1(filename: &str) {
    let mut ans = 0;
    let mut previous_depth: i32 = -1;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let current_depth = match s.parse::<i32>() {
                Ok(num) => { num },
                Err(e) => { panic!("[INPUT ERROR] {}", e); }
            };
            if previous_depth != -1 && previous_depth < current_depth {
                ans += 1;
            }
            previous_depth = current_depth;
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans = 0;
    let mut sliding_window: [i32; 3] = [0; 3];

    // Open file and read line by line
    let lines = read_lines(filename);
    for (index, line) in lines.enumerate() {
        if let Ok(s) = line {
            let current_depth = match s.parse::<i32>() {
                Ok(num) => { num },
                Err(e) => { panic!("[INPUT ERROR] {}", e); }
            };

            if index <= 2 {
                sliding_window[index] = current_depth;
            } else {
                let previous_window_sum = sliding_window.iter().sum::<i32>();
                sliding_window[index % 3] = current_depth;
                let current_window_sum = sliding_window.iter().sum::<i32>();
                if current_window_sum > previous_window_sum {
                    ans += 1;
                }
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
