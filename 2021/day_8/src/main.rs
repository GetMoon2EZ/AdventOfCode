use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::collections::HashMap;

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Debug, Clone)]
struct RawSignal {
    numbers: [String; 10],
    code: [String; 4],
}

impl From<&str> for RawSignal {
    fn from(s: &str) -> Self {
        let split_pipe = s.split(" | ").collect::<Vec<&str>>();
        if split_pipe.len() != 2 {
            panic!("[ERROR] Cannot convert {} to RawSignal: expected 1 pipe, got {}!", s, split_pipe.len() - 1);
        }
        let v_numbers = split_pipe[0].split(" ").collect::<Vec<&str>>();
        if v_numbers.len() != 10 {
            panic!("[ERROR] Cannot convert {} to RawSignal: expected 10 numbers before pipe, got {}!", s, v_numbers.len());
        }
        let mut numbers: [String; 10] = Default::default();
        for (i, num) in v_numbers.iter().enumerate() {
            numbers[i] = String::from(*num);
        }

        let v_code = split_pipe[1].split(" ").collect::<Vec<&str>>();
        if v_code.len() != 4 {
            panic!("[ERROR] Cannot convert {} to RawSignal: expected 4 numbers after pipe, got {}!", s, v_code.len());
        }
        let mut code: [String; 4] = Default::default();
        for (i, num) in v_code.iter().enumerate() {
            code[i] = String::from(*num);
        }

        Self {
            numbers,
            code,
        }
    }
}

impl RawSignal {
    fn decode(&self) -> DecodedSignal {
        let mut str_to_number: HashMap<String, u8> = HashMap::new();
        let mut number_to_str: HashMap<u8, String> = HashMap::new();

        // Find 1, 4, 7, 8
        for segments in self.numbers.iter() {
            let sorted_segments = sort_string(segments.to_string());
            match sorted_segments.len() {
                2 => {
                    str_to_number.insert(sorted_segments.clone(), 1);
                    number_to_str.insert(1, sorted_segments);
                },
                3 => {
                    str_to_number.insert(sorted_segments.clone(), 7);
                    number_to_str.insert(7, sorted_segments);
                },
                4 => {
                    str_to_number.insert(sorted_segments.clone(), 4);
                    number_to_str.insert(4, sorted_segments);
                },
                7 => {
                    str_to_number.insert(sorted_segments.clone(), 8);
                    number_to_str.insert(8, sorted_segments);
                },
                _ => { },
            }
        }

        for sorted_segments in self.numbers.iter().map(|s| sort_string(s.to_string())) {
            // Find 3 -> len == 5 and includes 7
            if sorted_segments.len() == 5 && is_included(number_to_str.get(&7).unwrap().to_string(), sorted_segments.clone())
            {
                str_to_number.insert(sorted_segments.clone(), 3);
                number_to_str.insert(3, sorted_segments);
            } else if sorted_segments.len() == 6 {
                if is_included(number_to_str.get(&4).unwrap().to_string(), sorted_segments.clone()) {
                    // Found 9
                    str_to_number.insert(sorted_segments.clone(), 9);
                    number_to_str.insert(9, sorted_segments);
                } else {
                    // 6 or 0
                    if is_included(number_to_str.get(&7).unwrap().to_string(), sorted_segments.clone()) {
                        // Found 0
                        str_to_number.insert(sorted_segments.clone(), 0);
                        number_to_str.insert(0, sorted_segments);
                    } else {
                        // Found 6
                        str_to_number.insert(sorted_segments.clone(), 6);
                        number_to_str.insert(6, sorted_segments);
                    }
                }
            }

        }

        for sorted_segments in self.numbers.iter().map(|s| sort_string(s.to_string())) {
            match str_to_number.get(&sorted_segments.clone()) {
                None => {
                    // 2 or 5 (5 is included in 9, 2 is not)
                    if is_included(sorted_segments.clone(), number_to_str.get(&9).unwrap().to_string()) {
                        str_to_number.insert(sorted_segments.clone(), 5);
                        number_to_str.insert(5, sorted_segments);
                    } else {
                        str_to_number.insert(sorted_segments.clone(), 2);
                        number_to_str.insert(2, sorted_segments);
                    }
                },
                _ => {
                    // Already known, nothing to do
                },
            }
        }

        // decode the code
        let mut code = [255; 4];
        for i in 0..self.code.len() {
            match str_to_number.get(&sort_string(self.code[i].clone())) {
                Some(&num) => { code[i] = num; },
                None => { code[i] = 255; },
            }
        }

        DecodedSignal {
            // str_to_number,
            // number_to_str,
            code,
        }
    }
}

#[derive(Debug)]
struct DecodedSignal {
    // str_to_number: HashMap<String, u8>,
    // number_to_str: HashMap<u8, String>,
    code: [u8; 4],
}

impl DecodedSignal {
    fn count_decoded_easily(&self) -> i32 {
        let mut count = 0;
        for num in self.code.iter() {
            if *num == 1 || *num == 4 || *num == 7 || *num == 8 {
                count += 1;
            }
        }
        return count;
    }

    fn get_code(&self) -> i32 {
        let mut code: i32 = 0;
        for digit in self.code.iter() {
            code *= 10;
            code += *digit as i32;
        }
        return code;
    }
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn sort_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    chars.into_iter().collect::<String>()
}

fn is_included(needle: String, haystack: String) -> bool{
    let mut found = true;
    let mut chars = haystack.chars().peekable();
    for character in needle.chars() {
        if !found {
            return false;
        }
        found = false;
        while chars.peek().is_some() {
            if character == chars.next().unwrap() {
                found = true;
                break;
            }
        }
    }
    return found;
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let raw_signal = RawSignal::from(s.as_str());
            let decoded_signal = raw_signal.decode();
            ans += decoded_signal.count_decoded_easily();
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
            let raw_signal = RawSignal::from(s.as_str());
            let decoded_signal = raw_signal.decode();
            let code = decoded_signal.get_code();
            ans += code;
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
