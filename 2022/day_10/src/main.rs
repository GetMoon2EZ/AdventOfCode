use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Debug, Copy, Clone)]
struct Processor {
    cycle_count: i32,
    registry: i32,
}

impl Processor {
    fn signal_strength(&self) -> i32 {
        return (self.cycle_count) * self.registry
    }
}

#[derive(Debug, Copy, Clone)]
enum ProcessorInstruction {
    AddX(i32),
    Noop,
}

impl From<&str> for ProcessorInstruction {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<&str>>();
        if split.len() > 2 {
            panic!("[ERROR] {} is not a valid CPU instruction!", s);
        }

        if split.len() == 2 {
            if split[0] != "addx" {
                panic!("[ERROR] {} is not a valid instruction!", s);
            }
            let amount = match split[1].parse::<i32>() {
                Ok(value) => { value },
                Err(e) => { panic!("[ERROR] {}", e); }
            };
            return Self::AddX(amount);
        }

        if split[0] != "noop" {
            panic!("[ERROR] {} is not a valid instruction!", s);
        }
        return Self::Noop;
    }
}

#[derive(Debug, Copy, Clone)]
struct Screen {
    pixels: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: [['.'; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn display(&self) {
        for row in 0..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                print!("{}", self.pixels[row][col]);
            }
            print!("\n");
        }
    }

    fn update_screen(&mut self, cpu: Processor) {
        if cpu.cycle_count > 240 {
            return;
        }
        let cycle_count = cpu.cycle_count - 1;

        // row and col of current screen pixel to draw
        let row = cycle_count / 40;
        let col = cycle_count % 40;

        if cpu.registry >= col - 1 && cpu.registry <= col + 1 {
            self.pixels[row as usize][col as usize] = '#';
        }

        #[cfg(debug_assertions)]
        println!("cpu.cycle = {}, cpu.registry {} : pixels[{}][{}] = '{}'", cpu.cycle_count, cpu.registry, row, col, self.pixels[row as usize][col as usize]);
    }
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn get_signal_strength(cpu: Processor) -> i32 {
    if ((cpu.cycle_count - 20) % 40) == 0 && cpu.cycle_count <= 220 {
        #[cfg(debug_assertions)]
        println!("Cycle n°{} : {} * {} = {}", cpu.cycle_count, cpu.cycle_count, cpu.registry, cpu.signal_strength());
        return cpu.signal_strength();
    }
    return 0;
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    let mut cpu = Processor {
        cycle_count: 1,
        registry: 1,
    };

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let instruction = ProcessorInstruction::from(s.as_str());
            match instruction {
                ProcessorInstruction::Noop => {
                    cpu.cycle_count += 1;
                    ans += get_signal_strength(cpu);
                },
                ProcessorInstruction::AddX(amount) => {
                    cpu.cycle_count += 1;
                    ans += get_signal_strength(cpu);
                    cpu.cycle_count += 1;
                    cpu.registry += amount;
                    ans += get_signal_strength(cpu);
                }
            }
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut screen = Screen::new();
    let mut cpu = Processor {
        cycle_count: 1,
        registry: 1,
    };

    screen.update_screen(cpu);

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let instruction = ProcessorInstruction::from(s.as_str());
            match instruction {
                ProcessorInstruction::Noop => {
                    cpu.cycle_count += 1;
                    screen.update_screen(cpu);
                },
                ProcessorInstruction::AddX(amount) => {
                    cpu.cycle_count += 1;
                    screen.update_screen(cpu);
                    cpu.cycle_count += 1;
                    cpu.registry += amount;
                    screen.update_screen(cpu);
                }
            }
        }
    }

    screen.display();
}

fn main() {
    let arg = Arg::parse();

    match arg.challenge_num {
        1 => { solve_problem_1(&arg.filename); },
        2 => { solve_problem_2(&arg.filename); },
        n => { panic!("[ERROR] Incorrect challenge number {}", n); }
    }
}
