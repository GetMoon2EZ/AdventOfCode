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

enum Mode {
    OxygenGeneratorRateMode,
    Co2ScrubberRateMode,
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn i32_from_bit_string(s: &str) -> i32 {
    let mut res = 0;
    for bit in s.chars() {
        res <<= 1;
        if bit == '1' {
            res += 1;
        }
    }
    return res;
}

fn get_rate(mut possible_values: Vec<String>, mode: Mode) -> i32{
    let mut index = 0;
    while possible_values.len() > 1 {
        // Filter ones at position "index"
        let ones: Vec<String> = possible_values.iter()
            .filter(
                |s| s.chars().nth(index) == Some('1')
            ).map(|s|String::from(s))
            .collect();

        // Filter zeros at position "index"
        let zeros: Vec<String> = possible_values.iter()
            .filter(
                |s| s.chars().nth(index) == Some('0')
            ).map(|s|String::from(s))
            .collect();

        // Find the most common bit
        if ones.len() >= zeros.len() {
            match mode {
                Mode::OxygenGeneratorRateMode => { possible_values = ones; },
                Mode::Co2ScrubberRateMode => { possible_values = zeros },
            }
        } else {
            match mode {
                Mode::OxygenGeneratorRateMode => { possible_values = zeros; },
                Mode::Co2ScrubberRateMode => { possible_values = ones },
            }
        }
        index += 1;
    }

    i32_from_bit_string(&possible_values[0])
}

fn solve_problem_1(filename: &str) {
    // Counts the number of 1s at each position
    let mut counter = Vec::new();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            for (index, bit) in s.chars().enumerate() {
                if index >= counter.len() {
                    counter.push(0);
                }
                if bit == '1' {
                    counter[index] += 1;
                } else if bit == '0' {
                    counter[index] -= 1;
                }
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for count in counter.into_iter() {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if count > 0 {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }
    }

    let ans = gamma_rate * epsilon_rate;
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut possible_values = Vec::new();
    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            possible_values.push(s);
        }
    }

    let oxygen_rate = get_rate(possible_values.clone(), Mode::OxygenGeneratorRateMode);
    let co2_rate = get_rate(possible_values, Mode::Co2ScrubberRateMode);

    let ans = oxygen_rate * co2_rate;
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
