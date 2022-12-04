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

#[derive(Debug, Copy, Clone)]
struct BingoTile {
    number: i32,
    drawn: bool,
}

#[derive(Debug, Clone)]
struct BingoGrid {
    tiles: [[BingoTile; 5]; 5],
}

impl BingoGrid {
    fn new() -> Self {
        let tile = BingoTile {
            number: 0,
            drawn: false,
        };
        BingoGrid {
            tiles: [[tile; 5]; 5],
        }
    }

    fn set_row(&mut self, row_index: usize, s: &str) {
        let mut col_index = 0;
        for num_str in s.split(" ") {
            if num_str.is_empty() {
                continue;
            }
            let value = match num_str.parse::<i32>() {
                Ok(num) => { num },
                Err(e) => { panic!("{}", e); }
            };
            self.tiles[row_index][col_index].number = value;
            self.tiles[row_index][col_index].drawn = false;
            col_index += 1;
        }
    }

    fn wins(&self) -> bool {
        let mut col_wins = [true; 5];
        for row in self.tiles.iter() {
            let mut row_wins = true;
            for (index, tile) in row.iter().enumerate() {
                if !tile.drawn {
                    row_wins = false;
                    col_wins[index] = false;
                }
            }
            if row_wins {
                return true;
            }
        }

        return col_wins.iter().any(|x| *x);
    }

    fn draw(&mut self, number: i32) {
        for row in self.tiles.iter_mut() {
            for tile in row.iter_mut() {
                if tile.number == number {
                    tile.drawn = true;
                }
            }
        }
    }

    fn get_score(&self, last_number_called: i32) -> i32 {
        let mut unmarked_sum = 0;
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if !tile.drawn {
                    unmarked_sum += tile.number;
                }
            }
        }
        return unmarked_sum * last_number_called;
    }
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => { f },
        Err(e) => { panic!("[ERROR] {}", e) }
    };
    BufReader::new(file).lines()
}

fn parse_file(filename: &str) -> (String, Vec<BingoGrid>) {
    let mut draw_order = String::from("");
    let mut grids = Vec::new();
    let mut row_number = 0;
    let mut current_grid = BingoGrid::new();


    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            if draw_order == "" {
                draw_order = s;
            } else if s.is_empty() {
                grids.push(current_grid.clone());
                row_number = 0;
            } else {
                current_grid.set_row(row_number, &s);
                row_number += 1;
            }
        }
    }
    grids.push(current_grid);
    grids.swap_remove(0);
    return (draw_order, grids);
}

fn solve_problem_1(filename: &str) {
    // Open file and read line by line
    let (draw_order, mut grids) = parse_file(filename);

    let mut ans = 0;
    // Insert number one by one
    for number in draw_order.split(",").map(|s| s.parse::<i32>().unwrap()) {
        let mut grid_won = false;
        for grid in grids.iter_mut() {
            grid.draw(number);
        }
        for grid in grids.iter() {
            if grid.wins() {
                ans = grid.get_score(number);
                grid_won = true;
            }
        }
        if grid_won {
            break;
        }
    }

    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let (draw_order, mut grids) = parse_file(filename);

    let mut ans = 0;
    // Insert number one by one
    for number in draw_order.split(",").map(|s| s.parse::<i32>().unwrap()) {
        for grid in grids.iter_mut() {
            grid.draw(number);
        }
        // Remove winning grids
        if grids.len() != 1 {
            grids = grids.into_iter().filter(|g| !g.wins()).collect();
        } else if grids[0].wins() {
            ans = grids[0].get_score(number);
            break;
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
