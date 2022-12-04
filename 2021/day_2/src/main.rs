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

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("[CONVERSION ERROR] Cannot convert {} into Command type", s);
        }

        let amount = match split[1].parse::<i32>() {
            Ok(value) => { value },
            Err(e) => { panic!("[INPUT ERROR] {}", e); }
        };

        match split[0] {
            "up" => { Self::Up(amount) },
            "down" => { Self::Down(amount) },
            "forward" => { Self::Forward(amount) },
            s => { panic!("[INPUT ERROR] Incorrect command: {}", s); }
        }
    }
}

struct Submarine {
    // Horizontal position
    x: i32,
    // Vertical position
    y: i32,
    // Aim
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            aim: 0,
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Up(amount) => {
                self.y -= amount;
            },
            Command::Down(amount) => {
                self.y += amount;
            },
            Command::Forward(amount) => {
                self.x += amount;
            },
        }
    }

    fn execute_command_with_aim(&mut self, command: Command) {
        match command {
            Command::Up(amount) => {
                self.aim -= amount;
            },
            Command::Down(amount) => {
                self.aim += amount;
            },
            Command::Forward(amount) => {
                self.x += amount;
                self.y += self.aim * amount;
            },
        }
    }

}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn solve_problem_1(filename: &str) {
    let mut submarine = Submarine::new();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let command = Command::from(s.as_str());
            submarine.execute_command(command);
        }
    }

    let ans = submarine.x * submarine.y;
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut submarine = Submarine::new();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let command = Command::from(s.as_str());
            submarine.execute_command_with_aim(command);
        }
    }

    let ans = submarine.x * submarine.y;
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
