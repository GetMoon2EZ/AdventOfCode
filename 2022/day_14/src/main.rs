use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use std::cmp::{min, max};

const SAND_SPAWN_POSITION: usize = 500;

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

struct Coordinates {
    x: usize,
    y: usize,
}

impl From<&str> for Coordinates {
    fn from(s: &str) -> Self {
        let values = s.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        if values.len() != 2 {
            panic!("[ERROR] Cannot convert {} to Coordinates", s);
        }

        Self {
            x: values[0],
            y: values[1],
        }
    }
}

#[derive(Clone, PartialEq)]
enum Element {
    Air,
    Rock,
    Sand,
}

#[cfg(debug_assertions)]
fn print_map(map: &Vec<Vec<Element>>) {
    let mut top = "┏".to_string();
    top.push_str((0..map[0].len()).map(|_| "━").collect::<String>().as_str());
    top.push_str("┓");
    let mut bottom = "┗".to_string();
    bottom.push_str((0..map[0].len()).map(|_| "━").collect::<String>().as_str());
    bottom.push_str("┛");
    println!("{}", top);

    for row in map.iter() {
        print!("┃");
        for unit in row.iter() {
            match unit {
                Element::Air => { print!("."); },
                Element::Rock => { print!("#"); },
                Element::Sand => { print!("o"); },
            }
        }
        print!("┃\n");
    }
    println!("{}", bottom);
}

fn parse_input(filename: &str, infinite_plane: bool) -> (Vec<Vec<Element>>, usize){
    // Read the file and build a Vec of Coordinates
    let mut rock_formations = Vec::new();
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            rock_formations.push(
                s.split(" -> ")
                    .map(|s| Coordinates::from(s))
                    .collect::<Vec<Coordinates>>()
                );
        }
    }

    // Find the lowest X in the Vec -> offset
    // Find the hightest X and Y (for map dimensions)
    let mut x_min = std::usize::MAX;
    let mut x_max = std::usize::MIN;
    let mut y_max = std::usize::MIN;
    for rock_formation in rock_formations.iter() {
        for coords in rock_formation.iter() {
            x_min = min(x_min, coords.x);
            x_max = max(x_max, coords.x);
            y_max = max(y_max, coords.y);
        }
    }

    if infinite_plane {
        y_max += 2;
        rock_formations.push(
            vec![
                Coordinates { x: SAND_SPAWN_POSITION - (y_max + 1), y: y_max },
                Coordinates { x: SAND_SPAWN_POSITION + (y_max + 1), y: y_max },
            ]
        );
        x_max = max(x_max, SAND_SPAWN_POSITION + (y_max + 1));
        x_min = min(x_min, SAND_SPAWN_POSITION - (y_max + 1));
    }
    let mut map: Vec<Vec<Element>> = vec![vec![Element::Air; (x_max + 1) - x_min]; y_max + 1];

    for rock_formation in rock_formations.iter() {
        for i in 0..rock_formation.len() - 1 {
            if rock_formation[i].x == rock_formation[i + 1].x {
                let x = rock_formation[i].x;
                let bottom = min(rock_formation[i].y, rock_formation[i + 1].y);
                let top = max(rock_formation[i].y, rock_formation[i + 1].y);
                for y in bottom..=top {
                    map[y][x - x_min] = Element::Rock;
                }
            } else {
                let y = rock_formation[i].y;
                let left = min(rock_formation[i].x, rock_formation[i + 1].x);
                let right = max(rock_formation[i].x, rock_formation[i + 1].x);
                for x in left..=right {
                    map[y][x - x_min] = Element::Rock;
                }
            }
        }
    }
    return (map, SAND_SPAWN_POSITION - x_min);
}

fn simulate_sand_pour(map: &mut Vec<Vec<Element>>, x_spawn: usize) -> u32 {
    let mut sand_count = 0;
    let y_max = map.len();
    let x_max = map[0].len();

    loop {
        let mut y_sand = 0;
        let mut x_sand = x_spawn;
        loop {
            if map[y_sand][x_sand] != Element::Air {
                return sand_count;
            }
            if y_sand + 1 >= y_max {
                return sand_count;
            }
            if map[y_sand + 1][x_sand] == Element::Air {
                y_sand += 1;
                continue;
            }
            if x_sand != 0 && map[y_sand + 1][x_sand - 1] == Element::Air {
                y_sand += 1;
                x_sand -= 1;
                continue;
            }

            if x_sand + 1 >= x_max || (x_sand == 0 && map[y_sand][x_sand + 1] != Element::Air) {
                // Falling on the right or the left
                return sand_count;
            }

            if x_sand + 1 != x_max && map[y_sand + 1][x_sand + 1] == Element::Air {
                y_sand += 1;
                x_sand += 1;
                continue;
            }
            break;
        }
        map[y_sand][x_sand] = Element::Sand;
        sand_count += 1;
        #[cfg(debug_assertions)]
        print_map(map);
    }
}

fn solve_problem_1(filename: &str) {
    let (mut map, x_sand) = parse_input(filename, false);

    #[cfg(debug_assertions)]
    print_map(&map);

    let ans = simulate_sand_pour(&mut map, x_sand);
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let (mut map, x_sand) = parse_input(filename, true);

    #[cfg(debug_assertions)]
    print_map(&map);

    let ans = simulate_sand_pour(&mut map, x_sand);
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
