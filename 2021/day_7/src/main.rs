use std::fs;
use clap::{Parser};
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Debug, Clone)]
struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn push_left(&mut self, number: i32) {
        self.left.push(number);
    }

    fn push_right(&mut self, number: i32) {
        self.right.push(Reverse(number));
    }

    fn peek_left(&self) -> i32 {
        match self.left.peek() {
            Some(&value) => { value },
            None => { 0 },
        }
    }

    fn peek_right(&self) -> i32 {
        match self.right.peek() {
            Some(Reverse(value)) => { *value },
            None => { std::i32::MAX },
        }
    }

    fn insert(&mut self, number: i32) {
        // Push to the left
        self.push_left(number);

        if self.left.len() > self.right.len() + 1 {
            self.push_right(self.peek_left());
            self.left.pop();
        }

        if self.peek_left() > self.peek_right() {
            self.push_right(self.peek_left());
            self.left.pop();
        }

        if self.left.len() + 1 < self.right.len() {
            self.push_left(self.peek_right());
            self.right.pop();
        }
    }

    fn get_median(&self) -> i32 {
        if self.left.len() == self.right.len() {
            return (self.peek_left() + self.peek_right()) / 2;
        } else if self.left.len() > self.right.len() {
            return self.peek_left();
        } else {
            return self.peek_right();
        }
    }
}

fn int_sum_to(number: i32) -> i32 {
    return (number * (number + 1)) / 2;
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    let contents = match fs::read_to_string(filename) {
        Ok(data) => { data },
        Err(e) => { panic!("{}", e); }
    };

    let mut median_finder = MedianFinder::new();
    for number in contents.split(",").map(|s| s.parse::<i32>().unwrap()) {
        median_finder.insert(number);
    }

    let median = median_finder.get_median();
    for value in median_finder.left.iter() {
        ans += (median - *value).abs();
    }
    for Reverse(value) in median_finder.right.iter() {
        ans += (median - *value).abs();
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let contents = match fs::read_to_string(filename) {
        Ok(data) => { data },
        Err(e) => { panic!("{}", e); }
    };

    let numbers: Vec<i32> = contents.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut sum = 0;
    for number in numbers.iter() {
        sum += number;
    }

    let average: i32 = sum / numbers.len() as i32;
    let mut ans_1 = 0;
    let mut ans_2 = 0;
    for number in numbers.iter() {
        ans_1 += int_sum_to((number - (average)).abs());
        ans_2 += int_sum_to((number - (average + 1)).abs());
    }

    let ans = min(ans_1, ans_2);
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
