use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const MAP_SIZE: usize = 25; // 25^3 = 15625

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapTileState {
    Lava,
    Air,
    Visited,
}

#[derive(Debug, Parser)]
struct Arg {
    /// Challenge to run (1 or 2)
    challenge_num: u8,

    /// Input file
    filename: String,
}

fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            panic!("[ERROR] {}", e)
        }
    };
    BufReader::new(file).lines()
}

fn parse_input(filename: &str) -> [[[MapTileState; MAP_SIZE]; MAP_SIZE]; MAP_SIZE] {
    // Instead of having a fixed sized map we could find the min and max values
    // in each dimension and create the smallest possible map thus saving some
    // memory (but I'm too lazy to do that right now)
    let mut map = [[[MapTileState::Air; MAP_SIZE]; MAP_SIZE]; MAP_SIZE];
    let lines = read_lines(filename);
    for line in lines {
        if let Ok(s) = line {
            let coords = s
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if coords.len() != 3 {
                panic!(
                    "[ERROR] Expecting 3D coordinates, got {}D coordinates !?",
                    coords.len()
                );
            }
            // Add 1 to coords to be sure that lava does not touch the edge of our map
            map[coords[2] + 1][coords[1] + 1][coords[0] + 1] = MapTileState::Lava;
        }
    }
    return map;
}

fn get_lava_surface_area_recursive(
    map: &mut [[[MapTileState; MAP_SIZE]; MAP_SIZE]; MAP_SIZE],
    x: usize,
    y: usize,
    z: usize,
) -> usize {
    let mut exposed_surface = 6;
    map[z][y][x] = MapTileState::Visited;
    // Check neighbors, starting with the x axis
    if x != 0 && map[z][y][x - 1] != MapTileState::Air {
        exposed_surface -= 1;
        if map[z][y][x - 1] != MapTileState::Visited {
            exposed_surface += get_lava_surface_area_recursive(map, x - 1, y, z);
        }
    }
    if x + 1 < MAP_SIZE && map[z][y][x + 1] != MapTileState::Air {
        exposed_surface -= 1;
        if map[z][y][x + 1] != MapTileState::Visited {
            exposed_surface += get_lava_surface_area_recursive(map, x + 1, y, z);
        }
    }

    // Then the y axis neighbors
    if y != 0 && map[z][y - 1][x] != MapTileState::Air {
        exposed_surface -= 1;
        if map[z][y - 1][x] != MapTileState::Visited {
            exposed_surface += get_lava_surface_area_recursive(map, x, y - 1, z);
        }
    }
    if y + 1 < MAP_SIZE && map[z][y + 1][x] != MapTileState::Air {
        exposed_surface -= 1;
        if map[z][y + 1][x] != MapTileState::Visited {
            exposed_surface += get_lava_surface_area_recursive(map, x, y + 1, z);
        }
    }

    // Finally, the z axis neighbors
    if z != 0 && map[z - 1][y][x] != MapTileState::Air {
        exposed_surface -= 1;
        if map[z - 1][y][x] != MapTileState::Visited {
            exposed_surface += get_lava_surface_area_recursive(map, x, y, z - 1);
        }
    }
    if z + 1 < MAP_SIZE && map[z + 1][y][x] != MapTileState::Air {
        exposed_surface -= 1;
        if map[z + 1][y][x] != MapTileState::Visited {
            exposed_surface += get_lava_surface_area_recursive(map, x, y, z + 1);
        }
    }

    return exposed_surface;
}

fn get_lava_surface_area(map: &mut [[[MapTileState; MAP_SIZE]; MAP_SIZE]; MAP_SIZE]) -> usize {
    // Find the lava
    let mut surface = 0;
    for z in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                if map[z][y][x] == MapTileState::Lava {
                    surface += get_lava_surface_area_recursive(map, x, y, z);
                }
            }
        }
    }
    // If no lava then surface area is 0
    return surface;
}

fn get_lava_exterior_surface_area_recursive(
    map: &mut [[[MapTileState; MAP_SIZE]; MAP_SIZE]; MAP_SIZE],
    x: usize,
    y: usize,
    z: usize,
) -> usize {
    // Check if any neighbor is lava if it is then add 1 to result
    // If a neighbor is Air then visit them
    let mut lava_area = 0;
    map[z][y][x] = MapTileState::Visited;

    if x != 0 {
        if map[z][y][x - 1] == MapTileState::Lava {
            lava_area += 1;
        } else if map[z][y][x - 1] == MapTileState::Air {
            lava_area += get_lava_exterior_surface_area_recursive(map, x - 1, y, z);
        }
    }
    if x + 1 < MAP_SIZE {
        if map[z][y][x + 1] == MapTileState::Lava {
            lava_area += 1;
        } else if map[z][y][x + 1] == MapTileState::Air {
            lava_area += get_lava_exterior_surface_area_recursive(map, x + 1, y, z);
        }
    }

    if y != 0 {
        if map[z][y - 1][x] == MapTileState::Lava {
            lava_area += 1;
        } else if map[z][y - 1][x] == MapTileState::Air {
            lava_area += get_lava_exterior_surface_area_recursive(map, x, y - 1, z);
        }
    }
    if y + 1 < MAP_SIZE {
        if map[z][y + 1][x] == MapTileState::Lava {
            lava_area += 1;
        } else if map[z][y + 1][x] == MapTileState::Air {
            lava_area += get_lava_exterior_surface_area_recursive(map, x, y + 1, z);
        }
    }

    if z != 0 {
        if map[z - 1][y][x] == MapTileState::Lava {
            lava_area += 1;
        } else if map[z - 1][y][x] == MapTileState::Air {
            lava_area += get_lava_exterior_surface_area_recursive(map, x, y, z - 1);
        }
    }
    if z + 1 < MAP_SIZE {
        if map[z + 1][y][x] == MapTileState::Lava {
            lava_area += 1;
        } else if map[z + 1][y][x] == MapTileState::Air {
            lava_area += get_lava_exterior_surface_area_recursive(map, x, y, z + 1);
        }
    }

    return lava_area;
}

fn get_lava_exterior_surface_area(
    map: &mut [[[MapTileState; MAP_SIZE]; MAP_SIZE]; MAP_SIZE],
) -> usize {
    // We know that map[0][0][0] is air because this is how we designed our map
    return get_lava_exterior_surface_area_recursive(map, 0, 0, 0);
}

fn solve_problem_1(filename: &str) {
    let mut map = parse_input(filename);
    let ans = get_lava_surface_area(&mut map);
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let mut map = parse_input(filename);
    let ans = get_lava_exterior_surface_area(&mut map);
    println!("Answer: {:?}", ans);
}

fn main() {
    let arg = Arg::parse();

    match arg.challenge_num {
        1 => {
            solve_problem_1(&arg.filename);
        }
        2 => {
            solve_problem_2(&arg.filename);
        }
        n => {
            panic!("[ERROR] Incorrect challenge number {}", n);
        }
    }
}
