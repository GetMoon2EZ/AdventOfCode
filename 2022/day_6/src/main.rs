use std::fs;
use clap::{Parser};

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

fn find_start_of_message_marker(filename: &str, start_marker_size: usize) -> i32 {
    let buffer = match fs::read_to_string(filename) {
        Ok(text) => { text },
        Err(e) => { panic!("[ERROR] {}", e); }
    };

    let mut ans: i32 = -1;
    for i in 0..(buffer.len() - start_marker_size) {
        let marker = &buffer[i..i + start_marker_size];
        let mut chars: Vec<char> = marker.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        chars.dedup();
        if chars.len() == start_marker_size {
            ans = (i + start_marker_size) as i32;
            break;
        }
    }
    return ans;
}

fn solve_problem_1(filename: &str) {
    let ans = find_start_of_message_marker(filename, 4);

    if ans == -1 {
        println!("No start signal");
    } else {
        println!("Answer: {}", ans);
    }
}

fn solve_problem_2(filename: &str) {
    let ans = find_start_of_message_marker(filename, 14);

    if ans == -1 {
        println!("No start signal");
    } else {
        println!("Answer: {}", ans);
    }
}

fn main() {
    let arg = Arg::parse();

    match arg.challenge_num {
        1 => { solve_problem_1(&arg.filename); },
        2 => { solve_problem_2(&arg.filename); },
        n => { panic!("[ERROR] Incorrect challenge number {}", n); }
    }
}
