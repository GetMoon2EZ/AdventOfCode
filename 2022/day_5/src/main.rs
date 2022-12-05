use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::str::FromStr;

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

#[derive(Debug, Copy, Clone)]
enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug, Clone)]
struct CrateStorage {
    stacks: Vec<Vec<char>>,
}

impl From<&Vec<String>> for CrateStorage {
    fn from(v: &Vec<String>) -> Self {
        // Get the number of stacks
        let count = match v.first() {
            Some(s) => { (s.len() + 1) / 4 },
            None => { panic!("[CONVERSION ERROR] Cannot convert {:?} to CrateStorage", v); }
        };
        // Initialize the stacks
        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..count {
            stacks.push(Vec::new());
        }

        // Parse each line in reverse order to get the stacks right
        for s in v.iter().rev() {
            for stack_index in 0..count {
                let character = match s.chars().nth(stack_index * 4 + 1) {
                    Some(character) => { character },
                    None => { panic!("[CONVERSION ERROR] Incorrect number of character in {}", s); }
                };
                if character.is_numeric() {
                    // This is the last line
                    break;
                } else if character != ' ' {
                    stacks[stack_index].push(character);
                }
            }
        }

        Self {
            stacks,
        }
    }
}

impl CrateStorage {
    fn use_crane(&mut self, crane_model: CraneModel, move_to_use: Move) {
        match crane_model {
            CraneModel::CrateMover9000 => {
                for _ in 0..move_to_use.amount {
                    let value = match self.stacks[move_to_use.from - 1].pop() {
                        Some(character) => { character },
                        None => { ' ' },
                    };
                    if value != ' ' {
                        self.stacks[move_to_use.to - 1].push(value);
                    }
                }
            },
            CraneModel::CrateMover9001 => {
                let mut tmp = Vec::new();
                for _ in 0..move_to_use.amount {
                    let value = match self.stacks[move_to_use.from - 1].pop() {
                        Some(character) => { character },
                        None => { ' ' },
                    };
                    tmp.push(value);
                }

                for character in tmp.into_iter().rev() {
                    self.stacks[move_to_use.to - 1].push(character);
                }
            }
        }
    }

    fn get_top(&self) -> String {
        self.stacks.iter()
            .map(|s| match s.last() {
                Some(value) => { return *value; },
                None => { return ' '; }
            })
            .collect::<String>()
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(" ").collect::<Vec<&str>>();
        if words.len() != 6 {
            return Err("Incorrect number of arguments".to_string());
        }

        let amount = match words[1].parse::<usize>() {
            Ok(number) => { number },
            Err(e) => { return Err(e.to_string()); }
        };
        let from = match words[3].parse::<usize>() {
            Ok(number) => { number },
            Err(e) => { return Err(e.to_string()); }
        };
        let to = match words[5].parse::<usize>() {
            Ok(number) => { number },
            Err(e) => { return Err(e.to_string()); }
        };

        Ok(
            Self {
                amount,
                from,
                to,
            }
        )
    }
}

fn parse_input(filename: &str) -> (CrateStorage, Vec<Move>) {
    let mut moves = Vec::new();
    let mut tmp_storage: Vec<String> = Vec::new();

    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            if s.is_empty() {
                continue;
            }
            if let Ok(next_move) = Move::from_str(s.as_str()) {
                moves.push(next_move);
            } else {
                tmp_storage.push(String::from(s));
            }
        }
    }

    let crate_storage = CrateStorage::from(&tmp_storage);
    return (crate_storage, moves);
}

fn solve_problem_1(filename: &str) {
    let (mut crate_storage, moves) = parse_input(filename);

    for current_move in moves.iter() {
        crate_storage.use_crane(CraneModel::CrateMover9000, *current_move);
    }

    let ans = crate_storage.get_top();
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let (mut crate_storage, moves) = parse_input(filename);

    for current_move in moves.iter() {
        crate_storage.use_crane(CraneModel::CrateMover9001, *current_move);
    }

    let ans = crate_storage.get_top();
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
