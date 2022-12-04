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

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn solve_problem_1(filename: &str) {
    let mut ans = 0;

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(mut s) = line {
            let r_comp = s.split_off(s.len() / 2);
            let l_comp = s;
            let mut items_map = HashMap::new();
            for item in l_comp.chars() {
                items_map.insert(item, true);
            }
            for item in r_comp.chars() {
                if items_map.contains_key(&item) {
                    if item.is_ascii_uppercase() {
                        ans += item.to_ascii_uppercase() as i32 - 'A' as i32 + 27;
                    } else {
                        ans += item.to_ascii_lowercase() as i32 - 'a' as i32 + 1;
                    }
                    break;
                }
            }
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut ans = 0;

    let mut line_cnt = 0;
    let mut group: [String; 3] = Default::default();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        let s = match line {
            Ok(txt) => { txt },
            Err(e) => { panic!("{}", e); }
        };
        group[line_cnt%3] = s;

        // Group is full
        if line_cnt % 3 == 2 {
            let mut items_map: HashMap<char, u8> = HashMap::new();
            // Fill the map with what the first elf owns
            for item in group[0].chars() {
                items_map.insert(item, 1);
            }

            // If elf 2 owns an item that's in the map then pass the count to 2
            for item in group[1].chars() {
                items_map.entry(item).and_modify(|count| *count = 2);
            }

            // If an item is owned by elf 3 and the other 2 elves then it is the badge
            for item in group[2].chars() {
                let value = match items_map.get(&item) {
                    Some(&v) => { v },
                    None => { 0 }
                };
                if value == 2 {
                    // println!("Badge: {}", item);
                    if item.is_ascii_uppercase() {
                        ans += item.to_ascii_uppercase() as i32 - 'A' as i32 + 27;
                    } else {
                        ans += item.to_ascii_lowercase() as i32 - 'a' as i32 + 1;
                    }
                    break;
                }
            }
        }
        line_cnt += 1;
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
