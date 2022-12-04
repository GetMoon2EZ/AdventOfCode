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
    let mut cal_cnt = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(ip) = line {
            let value = ip.parse::<i32>();
            match value {
                Ok(num) => { cal_cnt += num; },
                _ => { cal_cnt = 0; }
            }
        }

        if cal_cnt > ans {
            ans = cal_cnt;
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans_arr:[i32; 3] = [0; 3];
    let mut curr_cal = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(ip) = line {
            let value = ip.parse::<i32>();
            match value {
                // Add cal count to current elf calories
                Ok(num) => {
                    curr_cal += num;
                },
                // Update answer
                _ => {
                    for val in ans_arr.iter_mut() {
                        if *val < curr_cal {
                            let tmp = *val;
                            *val = curr_cal;
                            curr_cal = tmp;
                        }
                    }
                    curr_cal = 0;
                }
            }
        }
    }

    // Last elf is not followed by a blank line...
    for val in ans_arr.iter_mut() {
        if *val < curr_cal {
            let tmp = *val;
            *val = curr_cal;
            curr_cal = tmp;
        }
    }

    let ans = ans_arr.iter().sum::<i32>();
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
