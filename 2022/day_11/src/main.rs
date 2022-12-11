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

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl Operation {
    fn apply(&self, value: i64) -> i64 {
        match self {
            Operation::Add(x) => { return value + x; },
            Operation::Multiply(x) => { return value * x; },
            Operation::Square => { return value * value; },
        }
    }
}

#[derive(Debug)]
struct Test {
    // The divisor
    divisible_by: i64,
    // If true: pass to monkey ...
    if_true: usize,
    // If false: pass to monkey ...
    if_false: usize,
}

impl Test {
    fn eval(&self, value: i64) -> usize {
        if value % self.divisible_by == 0 {
            return self.if_true;
        } else {
            return self.if_false;
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    inspection_count: i64,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();
        if lines.len() != 7 {
            let err_msg: String = format!("[ERROR] Incorrect line number for a monkey: expected 7 got {}!", lines.len());
            return Err(err_msg);
        }
        // Get the items from the second line
        let items = parse_items(lines[1]);
        // Get the operation from the third line
        let operation = parse_operation(lines[2]);
        // Get the test from the other lines
        let test = parse_test(lines[3], lines[4], lines[5]);

        Ok(
            Self {
                items,
                operation,
                test,
                inspection_count: 0,
            }
        )
    }
}

fn parse_items(line: &str) -> Vec<i64> {
    line.strip_prefix("  Starting items: ").unwrap()
        .split(", ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_operation(line: &str) -> Operation {
    let op_split = line.strip_prefix("  Operation: new = old ").unwrap().split(" ").collect::<Vec<&str>>();
    let operation = if op_split[0] == "*" && op_split[1] == "old" {
        Operation::Square
    } else {
        let num = op_split[1].parse::<i64>().unwrap();
        match op_split[0] {
            "+" => { Operation::Add(num) },
            "*" => { Operation::Multiply(num) },
            _ => { Operation::Add(0) },
        }
    };
    return operation;
}

fn parse_test(test_line: &str, if_true_line: &str, if_false_line: &str) -> Test {
    let if_true = if_true_line.strip_prefix("    If true: throw to monkey ").unwrap().parse::<u32>().unwrap() as usize;
    let if_false = if_false_line.strip_prefix("    If false: throw to monkey ").unwrap().parse::<u32>().unwrap() as usize;
    let divisible_by = test_line.strip_prefix("  Test: divisible by ").unwrap().parse::<i64>().unwrap();

    Test {
        divisible_by,
        if_true,
        if_false,
    }
}

fn get_monkey_business_level(monkeys: &Vec<Monkey>, n: i32) -> i64{
    let mut best: Vec<i64> = vec![0; n as usize];

    for monkey in monkeys.iter() {
        let mut value = monkey.inspection_count;
        for val in best.iter_mut() {
            if *val < value {
                let tmp = *val;
                *val = value;
                value = tmp;
            }
        }
    }

    return best.iter().fold(1, |res, n| res * n);
}

fn simulate_rounds(monkeys: &mut Vec<Monkey>, n: i32, relief: bool) {
    let product = monkeys.iter().fold(1, |res, m| res * m.test.divisible_by);

    for _ in 0..n {
        for i_monkey in 0..monkeys.len() {
            monkeys[i_monkey].inspection_count += monkeys[i_monkey].items.len() as i64;
            // Go though each item for current monkey
            let mut i = 0;
            while i < monkeys[i_monkey].items.len() {
                let mut item = monkeys[i_monkey].items[i];
                item = monkeys[i_monkey].operation.apply(item);
                if relief {
                    item /= 3;
                } else {
                    item = item % product;
                }
                let give_to = monkeys[i_monkey].test.eval(item);
                if give_to != i_monkey {
                    monkeys[give_to].items.push(item);
                    monkeys[i_monkey].items.remove(i);
                    continue;
                }
                i += 1;
            }
        }
    }
}

fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut buffer: String = Default::default();
    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            if !s.is_empty() {
                buffer.push_str(s.as_str());
                buffer.push_str("\n");
            }
        }
        if let Ok(monkey) = Monkey::from_str(buffer.as_str()) {
            monkeys.push(monkey);
            // Reset buffer
            buffer = String::from("");
        }
    }
    return monkeys;
}

fn solve_problem_1(filename: &str) {
    let mut monkeys = parse_monkeys(filename);
    // Simulate 20 rounds
    simulate_rounds(&mut monkeys, 20, true);
    let ans = get_monkey_business_level(&monkeys, 2);

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut monkeys = parse_monkeys(filename);
    // Simulate 10000 rounds
    simulate_rounds(&mut monkeys, 10000, false);
    let ans = get_monkey_business_level(&monkeys, 2);

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
