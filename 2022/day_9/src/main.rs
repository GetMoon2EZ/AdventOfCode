use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::collections::HashMap;

#[cfg(debug_assertions)]
const PRINT_MIN_X: i32 = -11;
#[cfg(debug_assertions)]
const PRINT_MAX_X: i32 = 10;
#[cfg(debug_assertions)]
const PRINT_MIN_Y: i32 = -5;
#[cfg(debug_assertions)]
const PRINT_MAX_Y: i32 = 15;

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Movement {
    amount: i32,
    direction: Direction,
}

impl ToString for Movement {
    fn to_string(&self) -> String {
        match self.direction {
            Direction::Up => {
                return "U ".to_owned() + &self.amount.to_string();
            }
            Direction::Down => {
                return "D ".to_owned() + &self.amount.to_string();
            }
            Direction::Left => {
                return "L ".to_owned() + &self.amount.to_string();
            }
            Direction::Right => {
                return "R ".to_owned() + &self.amount.to_string();
            }
        };
    }
}

impl From<&str> for Movement {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<&str>>();
        if split.len() != 2 {
            panic!("[ERROR] Cannot convert {} to Movement", s);
        }

        let amount = match split[1].parse::<i32>() {
            Ok(value) => { value },
            Err(e) => { panic!("[ERROR] {}", e); }
        };

        let direction = match split[0] {
            "U" => { Direction::Up },
            "D" => { Direction::Down },
            "L" => { Direction::Left },
            "R" => { Direction::Right },
            other => { panic!("[ERROR] {} is not a valid direction indicator", other); }
        };

        Self {
            amount,
            direction,
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Coordinates>,
}

impl Rope {
    fn new(knot_count: i32) -> Self {
        let mut knots = Vec::new();
        for _ in 0..knot_count {
            knots.push(
                Coordinates {
                    x: 0,
                    y: 0,
                }
            );
        }
        Self {
            knots,
        }
    }

    #[cfg(debug_assertions)]
    fn print_position(&self) {
        for y in (PRINT_MIN_Y..=PRINT_MAX_Y).rev() {
            for x in PRINT_MIN_X..=PRINT_MAX_X {
                let mut character = '.';
                for i in (0..self.knots.len()).rev() {
                    if self.knots[i].x == x && self.knots[i].y == y {
                        if i == 0 {
                            character = 'H';
                        } else if i == self.knots.len() - 1 {
                            character = 'T';
                        } else {
                            character = char::from_digit(i as u32, 10).unwrap();
                        }
                    }
                }
                print!("{}", character);
            }
            print!("\n");
        }
        println!("");
    }

    fn apply_movement(&mut self, map: &mut HashMap<Coordinates, bool>, movement: Movement) {
        #[cfg(debug_assertions)]
        println!("== {} ==", movement.to_string());

        for _ in 0..movement.amount {
            // Update head position
            match movement.direction {
                Direction::Up => {
                    self.knots[0].y += 1;
                },
                Direction::Down => {
                    self.knots[0].y -= 1;
                },
                Direction::Left => {
                    self.knots[0].x -= 1;
                },
                Direction::Right => {
                    self.knots[0].x += 1;
                },
            }
            // Update rest of the rope
            map.entry(self.follow_head()).or_insert(true);

            #[cfg(debug_assertions)]
            self.print_position();
        }
    }

    fn follow_head(&mut self) -> Coordinates {
        for i in 1..self.knots.len() {
            if self.knots[i-1].x == self.knots[i].x {
                // Same row
                if self.knots[i].y > self.knots[i - 1].y + 1 {
                    self.knots[i].y -= 1;
                } else if self.knots[i].y < self.knots[i - 1].y - 1 {
                    self.knots[i].y += 1;
                }
            } else if self.knots[i - 1].y == self.knots[i].y {
                // Same column
                if self.knots[i].x > self.knots[i - 1].x + 1 {
                    self.knots[i].x -= 1;
                } else if self.knots[i].x < self.knots[i - 1].x - 1 {
                    self.knots[i].x += 1;
                }
            } else if (self.knots[i - 1].y - self.knots[i].y).abs() > 1 {
                if (self.knots[i - 1].x - self.knots[i].x).abs() > 1 {
                    self.knots[i].x += (self.knots[i - 1].x - self.knots[i].x)/2;
                } else {
                    self.knots[i].x = self.knots[i - 1].x;
                }
                if self.knots[i - 1].y > self.knots[i].y {
                    self.knots[i].y += 1;
                } else if self.knots[i - 1].y < self.knots[i].y {
                    self.knots[i].y -= 1;
                }
            } else if (self.knots[i - 1].x - self.knots[i].x).abs() > 1 {
                self.knots[i].y = self.knots[i - 1].y;
                if self.knots[i - 1].x > self.knots[i].x {
                    self.knots[i].x += 1;
                } else if self.knots[i - 1].x < self.knots[i].x {
                    self.knots[i].x -= 1;
                }
            }
        }
        return *self.knots.last().unwrap();
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
    let mut visited: HashMap<Coordinates, bool> = HashMap::new();
    let mut rope = Rope::new(2);
    visited.insert(*rope.knots.last().unwrap(), true);

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let movement = Movement::from(s.as_str());
            rope.apply_movement(&mut visited, movement);
        }
    }

    let ans = visited.len();
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut visited: HashMap<Coordinates, bool> = HashMap::new();
    let mut rope = Rope::new(10);
    visited.insert(*rope.knots.last().unwrap(), true);

    #[cfg(debug_assertions)]
    println!("== Initial State ==");
    #[cfg(debug_assertions)]
    rope.print_position();

    // Open file and read line by line
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let movement = Movement::from(s.as_str());
            rope.apply_movement(&mut visited, movement);
        }
    }

    #[cfg(debug_assertions)]
    for y in (PRINT_MIN_Y..PRINT_MAX_Y).rev() {
        for x in PRINT_MIN_X..PRINT_MAX_X {
            let character = match visited.get(&Coordinates { x, y }) {
                Some(_) => { '#' },
                None => { '.' },
            };
            print!("{}", character);
        }
        print!("\n");
    }

    let ans = visited.len();
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
