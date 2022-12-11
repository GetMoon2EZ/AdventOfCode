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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Opening {
    Parenthesis,
    Bracket,
    CurlyBracket,
    AngleBracket,
}

impl Opening {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '(' => { Ok(Self::Parenthesis) },
            '[' => { Ok(Self::Bracket) },
            '{' => { Ok(Self::CurlyBracket) },
            '<' => { Ok(Self::AngleBracket) },
            _ => { Err(format!("[ERROR] Cannot convert '{}' to Opening type", c)) },
        }
    }

    fn get_closing(&self) -> Closing {
        match self {
            Self::Parenthesis => { Closing::Parenthesis },
            Self::Bracket => { Closing::Bracket },
            Self::CurlyBracket => { Closing::CurlyBracket },
            Self::AngleBracket => { Closing::AngleBracket },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Closing {
    Parenthesis,
    Bracket,
    CurlyBracket,
    AngleBracket,
}

impl Closing {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            ')' => { Ok(Self::Parenthesis) },
            ']' => { Ok(Self::Bracket) },
            '}' => { Ok(Self::CurlyBracket) },
            '>' => { Ok(Self::AngleBracket) },
            _ => { Err(format!("[ERROR] Cannot convert '{}' to Closing type", c)) },
        }
    }

    fn get_corruption_score(&self) -> u64 {
        match self {
            Self::Parenthesis => { 3 },
            Self::Bracket => { 57 },
            Self::CurlyBracket => { 1197 },
            Self::AngleBracket => { 25137 },
        }
    }

    fn get_autocompletion_score(&self) -> u64 {
        match self {
            Self::Parenthesis => { 1 },
            Self::Bracket => { 2 },
            Self::CurlyBracket => { 3 },
            Self::AngleBracket => { 4 },
        }
    }
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let mut stack: Vec<Opening> = Vec::new();
            for character in s.chars() {
                if let Ok(opening) = Opening::from_char(character) {
                    stack.push(opening);
                } else if let Ok(closing) = Closing::from_char(character) {
                    match stack.pop() {
                        Some(opening) => {
                            if opening.get_closing() != closing {
                                ans += closing.get_corruption_score();
                            }
                        }
                        None => { ans += closing.get_corruption_score(); }
                    };
                } else {
                    panic!("[Error] '{}' is not a valid delimiter!", character);
                }
            }
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans_array: Vec<u64> = Vec::new();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let mut stack: Vec<Opening> = Vec::new();
            for character in s.chars() {
                if let Ok(opening) = Opening::from_char(character) {
                    stack.push(opening);
                } else if let Ok(closing) = Closing::from_char(character) {
                    match stack.pop() {
                        Some(opening) => {
                            if opening.get_closing() != closing {
                                stack = Vec::new();
                                break;
                            }
                        }
                        None => {
                            stack = Vec::new();
                            break;
                        }
                    };
                } else {
                    panic!("[Error] '{}' is not a valid delimiter!", character);
                }
            }
            let mut score: u64 = 0;
            while let Some(opening) = stack.pop() {
                score *= 5;
                score += opening.get_closing().get_autocompletion_score();
            }
            if score != 0 {
                ans_array.push(score);
            }
        }
    }

    ans_array.sort_unstable();
    let ans = ans_array[ans_array.len()/2];
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
