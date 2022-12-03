use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};

const LOSING_SCORE: i32 = 0;
const DRAWING_SCORE: i32 = 3;
const WINNING_SCORE: i32 = 6;

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl Sign {
    fn get_score(&self) -> i32 {
        match self {
            Sign::Rock => { return ROCK_SCORE; },
            Sign::Paper => { return PAPER_SCORE; },
            Sign::Scissors => { return SCISSORS_SCORE; },
        }
    }

    fn wins_against(&self) -> Self {
        match self {
            Sign::Rock => { return Sign::Scissors; },
            Sign::Paper => { return Sign::Rock; },
            Sign::Scissors => { return Sign::Paper; },
        }
    }

    fn draws_against(&self) -> Self {
        *self
    }

    fn loses_against(&self) -> Self {
        match self {
            Sign::Rock => { return Sign::Paper; },
            Sign::Paper => { return Sign::Scissors; },
            Sign::Scissors => { return Sign::Rock; },
        }
    }

    fn against(&self, sign: Self) -> Outcome {
        if sign == self.wins_against() {
            return Outcome::Win;
        } else if sign == self.draws_against() {
            return Outcome::Draw;
        } else {
            return Outcome::Lose;
        }
    }
}

impl From<char> for Sign {
    fn from(c: char) -> Self {
        match c {
            'A' => { return Sign::Rock; },
            'B' => { return Sign::Paper; },
            'C' => { return Sign::Scissors; },
            'X' => { return Sign::Rock; },
            'Y' => { return Sign::Paper; },
            'Z' => { return Sign::Scissors; },
            _ => { panic!("[CONVERSION ERROR] Cannot convert '{}' to enum Sign", c); }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => { return Outcome::Lose; },
            'Y' => { return Outcome::Draw; },
            'Z' => { return Outcome::Win; },
            _ => { panic!("[CONVERSION ERROR] Cannot convert '{}' to enum Outcome", c); }
        }
    }
}

impl Outcome {
    fn get_score(&self) -> i32 {
        match self {
            Outcome::Lose => { return LOSING_SCORE; },
            Outcome::Draw => { return DRAWING_SCORE; },
            Outcome::Win => { return WINNING_SCORE; },
        }
    }

    fn against(&self, sign: Sign) -> Sign {
        match self {
            Outcome::Lose => {
                return sign.wins_against();
            },
            Outcome::Draw => {
                return sign.draws_against();
            },
            Outcome::Win => {
                return sign.loses_against();
            }
        }
    }
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            if s.len() == 3 {
                let opponent_sign = Sign::from(s.chars().nth(0).unwrap());
                let my_sign = Sign::from(s.chars().nth(2).unwrap());
                let score = my_sign.get_score() + my_sign.against(opponent_sign).get_score();
                ans += score;
            }
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            if s.len() == 3 {
                let opponent_sign = Sign::from(s.chars().nth(0).unwrap());
                let outcome = Outcome::from(s.chars().nth(2).unwrap());
                let score = outcome.get_score() + outcome.against(opponent_sign).get_score();
                ans += score;
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
