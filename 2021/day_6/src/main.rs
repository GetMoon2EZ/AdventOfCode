use std::fs;
use clap::{Parser};

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

/// Simulate for simulation_duration days
/// Return the number of fish by the end of the simulation
fn simulate(input: &str, simulation_duration: usize) -> u64 {
    /*
    Solution:
        * Have an array of u64, each value is the number of fish that reproduce that that day
            -> Each day, add arr[day%7] to new_born[day%9]
        * Have an array of u64
            -> Transfer new_born[day%9] to arr[day%7]
    Answer is the sum of every value from both array
    */

    let mut fish_count: [u64; 7] = [0; 7];
    let mut new_born_count: [u64; 9] = [0; 9];

    for value in input.split(",") {
        let timer = match value.parse::<usize>() {
            Ok(number) => { number },
            Err(e) => { panic!("{}", e); }
        };
        fish_count[timer] += 1;
    }

    for day in 0..simulation_duration {
        fish_count[day%7] += new_born_count[day%9];
        new_born_count[day%9] = fish_count[day%7];
    }

    return fish_count.iter().sum::<u64>() + new_born_count.iter().sum::<u64>();
}

fn solve_problem_1(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(data) => { data },
        Err(e) => { panic!("{}", e); }
    };
    let ans = simulate(&contents, 80);
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(data) => { data },
        Err(e) => { panic!("{}", e); }
    };
    let ans = simulate(&contents, 256);
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
