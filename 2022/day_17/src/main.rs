use std::fs::{File};
use std::io::{Read};
use clap::{Parser};
use std::cmp::{max};
use std::collections::HashMap;

const CHAMBER_WIDTH: usize = 7;
const ROCK_ORDER: [Shape; 5] = [Shape::Dash, Shape::Plus, Shape::ReverseL, Shape::Pipe, Shape::Square];

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Shape {
    Dash,
    Plus,
    ReverseL,
    Pipe,
    Square,
}

impl Shape {
    fn get_height(&self) -> usize {
        match self {
            Self::Dash => { 0 },
            Self::Plus => { 2 },
            Self::ReverseL => { 2 },
            Self::Pipe => { 3 },
            Self::Square => { 1 },
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
struct Chamber {
    content: Vec<[bool; CHAMBER_WIDTH]>,
    height: usize,
}

impl Chamber {
    fn new() -> Self {
        Self {
            content: Vec::new(),
            height: 0,
        }
    }

    fn init_rock(&mut self, rock: Shape) {
        match rock {
            Shape::Dash => {
                for i in 0..4 {
                    self.content[self.height + 3][2 + i] = true;
                }
            },
            Shape::Plus => {
                self.content[self.height + 3][3] = true;
                self.content[self.height + 4][2] = true;
                self.content[self.height + 4][3] = true;
                self.content[self.height + 4][4] = true;
                self.content[self.height + 5][3] = true;
            }
            Shape::ReverseL => {
                for i in 0..3 {
                    self.content[self.height + 3][2 + i] = true;
                }
                self.content[self.height + 4][4] = true;
                self.content[self.height + 5][4] = true;
            },
            Shape::Pipe => {
                for i in 0..4 {
                    self.content[self.height + 3 + i][2] = true;
                }
            },
            Shape::Square => {
                for i in 0..2 {
                    self.content[self.height + 3 + i][2] = true;
                    self.content[self.height + 3 + i][3] = true;
                }
            },
        }
    }

    fn drop_rock(&mut self, rock: Shape, moves: &Vec<Direction>, mut i_move: usize) -> usize {
        let mut rock_y = self.height + 3;
        let mut rock_x = 2;

        for _ in self.height..=self.height + rock.get_height() + 4 {
            self.content.push([false; CHAMBER_WIDTH]);
        }

        self.init_rock(rock);

        loop {
            let (prev_x, prev_y) = self.move_rock(rock, moves[i_move], rock_x, rock_y);
            (rock_x, rock_y) = self.move_rock(rock, Direction::Down, prev_x, prev_y);
            i_move = (i_move + 1) % moves.len();

            if rock_y == prev_y {
                self.height = max(self.height, rock_y + rock.get_height() + 1);
                break;
            }
        }
        return i_move;
    }

    fn move_rock(&mut self, rock: Shape, movement: Direction, x: usize, y: usize) -> (usize, usize) {
        match rock {
            Shape::Dash => { return self.move_rock_dash(movement, x, y); }
            Shape::Plus => { return self.move_rock_plus(movement, x, y); }
            Shape::ReverseL => { return self.move_rock_reverse_l(movement, x, y); }
            Shape::Pipe => { return self.move_rock_pipe(movement, x, y); }
            Shape::Square => { return self.move_rock_square(movement, x, y); }
        }
    }

    fn move_rock_dash(&mut self, movement: Direction, x: usize, y: usize) -> (usize, usize) {
        match movement {
            Direction::Down => {
                if y > 0 {
                    // Check movement
                    for rock_x in x..x+4 {
                        if self.content[y - 1][rock_x] {
                            return (x, y);
                        }
                    }
                    // Move
                    for rock_x in x..x+4 {
                        self.content[y][rock_x] = false;
                        self.content[y - 1][rock_x] = true;
                    }
                    return (x, y - 1);
                } else {
                    return (x, y);
                }
            },
            Direction::Left => {
                if x > 0 && !self.content[y][x - 1] {
                    self.content[y][x + 3] = false;
                    self.content[y][x - 1] = true;
                    return (x - 1, y);
                } else {
                    return (x, y);
                }
            },
            Direction::Right => {
                if x + 4 < CHAMBER_WIDTH && !self.content[y][x + 4] {
                    self.content[y][x] = false;
                    self.content[y][x + 4] = true;
                    return (x + 1, y);
                } else {
                    return (x, y);
                }
            },
        }
    }

    fn move_rock_plus(&mut self, movement: Direction, x: usize, y: usize) -> (usize, usize) {
        match movement {
            Direction::Down => {
                if y > 0 {
                    // Check movement
                    if
                        self.content[y - 1][x + 1] ||
                        self.content[y][x] ||
                        self.content[y][x + 2]
                    {
                        return (x, y);
                    }
                    // Move
                    self.content[y + 1][x] = false;
                    self.content[y + 2][x + 1] = false;
                    self.content[y + 1][x + 2] = false;

                    self.content[y][x] = true;
                    self.content[y - 1][x + 1] = true;
                    self.content[y][x + 2] = true;
                    return (x, y - 1);
                } else {
                    return (x, y);
                }
            },
            Direction::Left => {
                if x > 0 {
                    if
                        self.content[y + 1][x - 1] ||
                        self.content[y][x] ||
                        self.content[y + 2][x]
                    {
                        return (x, y)
                    }
                    /*
                    ..#..
                    .###.
                    .x#..
                    */
                    // Move
                    self.content[y][x + 1] = false;
                    self.content[y + 1][x + 2] = false;
                    self.content[y + 2][x + 1] = false;

                    self.content[y + 1][x - 1] = true;
                    self.content[y][x] = true;
                    self.content[y + 2][x] = true;
                    return (x - 1, y);
                } else {
                    return (x, y);
                }
            },
            Direction::Right => {
                if x + 3 < CHAMBER_WIDTH {
                    if
                        self.content[y][x + 2] ||
                        self.content[y + 1][x + 3] ||
                        self.content[y + 2][x + 2]
                    {
                        return (x, y)
                    }
                    self.content[y][x + 1] = false;
                    self.content[y + 1][x] = false;
                    self.content[y + 2][x + 1] = false;

                    self.content[y][x + 2] = true;
                    self.content[y + 1][x + 3] = true;
                    self.content[y + 2][x + 2] = true;
                    return (x + 1, y);
                } else {
                    return (x, y);
                }
            },
        }
    }

    fn move_rock_reverse_l(&mut self, movement: Direction, x: usize, y: usize) -> (usize, usize) {
        match movement {
            Direction::Down => {
                if y > 0 {
                    // Check movement
                    for rock_x in x..x+3 {
                        if self.content[y - 1][rock_x] {
                            return (x, y);
                        }
                    }
                    // Move
                    self.content[y][x] = false;
                    self.content[y][x + 1] = false;
                    self.content[y + 2][x + 2] = false;

                    self.content[y - 1][x] = true;
                    self.content[y - 1][x + 1] = true;
                    self.content[y - 1][x + 2] = true;

                    return (x, y - 1);
                } else {
                    return (x, y);
                }
            },
            Direction::Left => {
                if x > 0 {
                    if
                        self.content[y][x - 1] ||
                        self.content[y + 1][x + 1] ||
                        self.content[y + 2][x + 1]
                    {
                        return (x, y);
                    }
                    for rock_y in y..y+3 {
                        self.content[rock_y][x + 2] = false;
                    }
                    self.content[y][x - 1] = true;
                    self.content[y + 1][x + 1] = true;
                    self.content[y + 2][x + 1] = true;
                    return (x - 1, y);
                } else {
                    return (x, y);
                }
            },
            Direction::Right => {
                if x + 3 < CHAMBER_WIDTH {
                    if
                        self.content[y][x + 3] ||
                        self.content[y + 1][x + 3] ||
                        self.content[y + 2][x + 3]
                    {
                        return (x, y);
                    }
                    self.content[y][x] = false;
                    self.content[y + 1][x + 2] = false;
                    self.content[y + 2][x + 2] = false;

                    self.content[y][x + 3] = true;
                    self.content[y + 1][x + 3] = true;
                    self.content[y + 2][x + 3] = true;
                    return (x + 1, y);
                } else {
                    return (x, y);
                }
            },
        }
    }

    fn move_rock_pipe(&mut self, movement: Direction, x: usize, y: usize) -> (usize, usize) {
        match movement {
            Direction::Down => {
                if y > 0 && !self.content[y - 1][x] {
                    self.content[y + 3][x] = false;
                    self.content[y - 1][x] = true;
                    return (x, y - 1);
                } else {
                    return (x, y);
                }
            },
            Direction::Left => {
                if x > 0 {
                    for rock_y in y..y+4 {
                        if self.content[rock_y][x - 1] {
                            return (x, y);
                        }
                    }
                    for rock_y in y..y+4 {
                        self.content[rock_y][x] = false;
                        self.content[rock_y][x - 1] = true;
                    }
                    return (x - 1, y);
                } else {
                    return (x, y);
                }
            },
            Direction::Right => {
                if x + 1 < CHAMBER_WIDTH {
                    for rock_y in y..y+4 {
                        if self.content[rock_y][x + 1] {
                            return (x, y);
                        }
                    }
                    for rock_y in y..y+4 {
                        self.content[rock_y][x] = false;
                        self.content[rock_y][x + 1] = true;
                    }
                    return (x + 1, y);
                } else {
                    return (x, y);
                }
            },
        }
    }

    fn move_rock_square(&mut self, movement: Direction, x: usize, y: usize) -> (usize, usize) {
        match movement {
            Direction::Down => {
                if y > 0 {
                    if
                        self.content[y - 1][x] ||
                        self.content[y - 1][x + 1]
                    {
                        return (x, y);
                    }
                    self.content[y + 1][x] = false;
                    self.content[y + 1][x + 1] = false;

                    self.content[y - 1][x] = true;
                    self.content[y - 1][x + 1] = true;
                    return (x, y - 1);
                } else {
                    return (x, y);
                }
            },
            Direction::Left => {
                if x > 0 {
                    if
                        self.content[y][x - 1] ||
                        self.content[y + 1][x - 1]
                    {
                        return (x, y);
                    }
                    self.content[y + 1][x + 1] = false;
                    self.content[y][x + 1] = false;

                    self.content[y][x - 1] = true;
                    self.content[y + 1][x - 1] = true;
                    return (x - 1, y);
                } else {
                    return (x, y);
                }
            },
            Direction::Right => {
                if x + 2 < CHAMBER_WIDTH {
                    if
                        self.content[y][x + 2] ||
                        self.content[y + 1][x + 2]
                    {
                        return (x, y);
                    }
                    self.content[y][x] = false;
                    self.content[y + 1][x] = false;

                    self.content[y][x + 2] = true;
                    self.content[y + 1][x + 2] = true;
                return (x + 1, y);
                } else {
                    return (x, y);
                }
            },
        }
    }

    fn update_cache(&mut self, cache: &mut Cache, i_rock: usize, i_move: usize) {
        if self.height > 100 {
            let mut cache_arr = [[false; CHAMBER_WIDTH]; 100];
            for i in self.height - 100..self.height {
                cache_arr[i - (self.height - 100)] = self.content[i];
            }
            cache.insert((ROCK_ORDER[i_rock % 5], i_move, cache_arr), (i_rock, self.height));
        }
    }

    fn check_cache(&mut self, cache: &Cache, i_rock: &mut usize, i_move: usize, nb_rocks: usize) -> usize {
        if self.height <= 100 {
            return 0;
        }

        let mut cache_arr = [[false; CHAMBER_WIDTH]; 100];
        for i in self.height - 100..self.height {
            cache_arr[i - (self.height - 100)] = self.content[i];
        }

        match cache.get(&(ROCK_ORDER[*i_rock % 5], i_move, cache_arr)) {
            None => { 0 },
            Some((nb_rock, last_height)) => {
                let delta_rocks = *i_rock - nb_rock;
                let delta_height = self.height - last_height;
                let skippable = (nb_rocks - *i_rock) / delta_rocks;
                *i_rock += skippable * delta_rocks;
                return delta_height * skippable;
            }
        }
    }
}

fn parse_input(filename: &str) -> Vec<Direction> {
    let mut data = String::new();
    let mut f = File::open(filename).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    let mut moves = Vec::new();
    for character in data.chars() {
        match character {
            '<' => { moves.push(Direction::Left); },
            '>' => { moves.push(Direction::Right); },
            _ => { panic!("[ERROR] Unexpected move: {}", character); }
        }
    }

    return moves;
}

// (Shape + movement + last 100 rows) => (move number, chamber height)
type Cache = HashMap<(Shape, usize, [[bool; CHAMBER_WIDTH]; 100]), (usize, usize)>;

fn simulate(chamber: &mut Chamber, moves: &Vec<Direction>, nb_rocks: usize) {
    let mut i_rock = 0;
    let mut i_move = 0;
    let mut cache: Cache = HashMap::new();
    let mut total_height = 0;
    while i_rock < nb_rocks {
        // Drop rock
        i_move = chamber.drop_rock(ROCK_ORDER[i_rock % 5], moves, i_move);
        if total_height == 0 {
            let skipped_height = chamber.check_cache(&cache, &mut i_rock, i_move, nb_rocks);
            if skipped_height > 0 {
                total_height = skipped_height;
            }
        }
        // Update cache
        chamber.update_cache(&mut cache, i_rock, i_move);
        i_rock += 1;
    }
    chamber.height += total_height;
}

fn solve_problem_1(filename: &str) {
    let moves = parse_input(filename);
    let mut chamber = Chamber::new();
    simulate(&mut chamber, &moves, 2022);
    let ans = chamber.height;
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let moves = parse_input(filename);
    let mut chamber = Chamber::new();
    simulate(&mut chamber, &moves, 1000000000000);
    let ans = chamber.height;
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
