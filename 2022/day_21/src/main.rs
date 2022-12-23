use std::collections::HashMap;
use std::fs;

type MonkeyMap = HashMap<String, Monkey>;

enum Operator {
    Integer(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Unknown,
    Equals,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "+" => { Self::Plus },
            "-" => { Self::Minus },
            "*" => { Self::Multiply },
            "/" => { Self::Divide },
            _ => { Self::Unknown }, // Should never happen
        }
    }
}

impl Monkey {
    fn can_calculate(&self, map: &MonkeyMap) -> bool {
        match self.operator {
            Operator::Integer(_) => { true },
            Operator::Unknown => { false },
            _ => {
                map.get(&self.left).unwrap().can_calculate(map) &&
                map.get(&self.right).unwrap().can_calculate(map)
            }
        }
    }

    fn solve(&self, map: &MonkeyMap) -> i64 {
        match self.operator {
            Operator::Integer(x) => { x },
            Operator::Plus => {
                map.get(&self.left).unwrap().solve(map) +
                map.get(&self.right).unwrap().solve(map)
            }
            Operator::Minus => {
                map.get(&self.left).unwrap().solve(map) -
                map.get(&self.right).unwrap().solve(map)
            }
            Operator::Multiply => {
                map.get(&self.left).unwrap().solve(map) *
                map.get(&self.right).unwrap().solve(map)
            }
            Operator::Divide => {
                map.get(&self.left).unwrap().solve(map) /
                map.get(&self.right).unwrap().solve(map)
            }
            _ => {
                panic!("Cannot solve for X");
            }
        }
    }

    fn solve_for_x_recursive(&self, target: i64, map: &MonkeyMap) -> i64 {
        match self.operator {
            Operator::Integer(x) => { x },
            Operator::Plus => {
                let left = map.get(&self.left).unwrap();
                let right = map.get(&self.right).unwrap();
                if left.can_calculate(map) {
                    let x = left.solve(map);
                    let new_target = target - x;
                    return right.solve_for_x_recursive(new_target, map);
                } else {
                    let x = right.solve(map);
                    let new_target = target - x;
                    return left.solve_for_x_recursive(new_target, map);
                }
            },
            Operator::Minus => {
                let left = map.get(&self.left).unwrap();
                let right = map.get(&self.right).unwrap();
                if left.can_calculate(map) {
                    let x = left.solve(map);
                    let new_target = x - target;
                    return right.solve_for_x_recursive(new_target, map);
                } else {
                    let x = right.solve(map);
                    let new_target = x + target;
                    return left.solve_for_x_recursive(new_target, map);
                }
            },
            Operator::Multiply => {
                let left = map.get(&self.left).unwrap();
                let right = map.get(&self.right).unwrap();
                if left.can_calculate(map) {
                    let x = left.solve(map);
                    let new_target = target / x;
                    return right.solve_for_x_recursive(new_target, map);
                } else {
                    let x = right.solve(map);
                    let new_target = target / x;
                    return left.solve_for_x_recursive(new_target, map);
                }
            },
            Operator::Divide => {
                let left = map.get(&self.left).unwrap();
                let right = map.get(&self.right).unwrap();
                if left.can_calculate(map) {
                    let x = left.solve(map);
                    let new_target = x / target;
                    return right.solve_for_x_recursive(new_target, map);
                } else {
                    let x = right.solve(map);
                    let new_target = x * target;
                    return left.solve_for_x_recursive(new_target, map);
                }
            }
            Operator::Unknown => { target },
            _ => { panic!("Should not happen"); },
        }
    }
}

struct Monkey {
    left: String,
    right: String,
    operator: Operator,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let value = s.parse::<i64>();
        match value {
            Ok(num) => {
                Self {
                    left: String::from(""),
                    right: String::from(""),
                    operator: Operator::Integer(num),
                }
            }
            Err(_) => {
                let split = s.split(" ").collect::<Vec<&str>>();
                Self {
                    left: String::from(split[0]),
                    right: String::from(split[2]),
                    operator: Operator::from(split[1]),
                }
            }
        }
    }
}

fn parse_input(filename: &str) -> MonkeyMap {
    let content: String = fs::read_to_string(filename).unwrap();
    let mut map = MonkeyMap::new();

    for line in content.lines() {
        let split = line.split(": ").collect::<Vec<&str>>();
        let name = String::from(split[0]);
        let monkey = Monkey::from(split[1]);
        map.insert(name, monkey);
    }

    return map;
}

fn modify_map(map: &mut MonkeyMap) {
    let s_root = String::from("root");
    let mut root = map.remove(&s_root).unwrap();
    root.operator = Operator::Equals;
    map.insert(s_root, root);

    let s_humn = String::from("humn");
    let mut humn = map.remove(&s_humn).unwrap();
    humn.operator = Operator::Unknown;
    map.insert(s_humn, humn);
}

fn solve_for_x(map: &MonkeyMap) -> i64 {
    let root = map.get(&String::from("root")).unwrap();
    let left = map.get(&root.left).unwrap();
    let right = map.get(&root.right).unwrap();
    if left.can_calculate(map) {
        let target = left.solve(map);
        right.solve_for_x_recursive(target, map)
    } else {
        let target = right.solve(map);
        left.solve_for_x_recursive(target, map)
    }
}

fn main() {
    // Part 1
    let map = parse_input("test.txt");
    let ans = map.get(&String::from("root")).unwrap().solve(&map);
    println!("Control : {}", ans);

    let map = parse_input("input.txt");
    let ans = map.get(&String::from("root")).unwrap().solve(&map);
    println!("Answer : {}", ans);

    // Part 2
    let mut map = parse_input("test.txt");
    modify_map(&mut map);
    let ans = solve_for_x(&map);
    println!("Control : {}", ans);

    let mut map = parse_input("input.txt");
    modify_map(&mut map);
    let ans = solve_for_x(&map);
    println!("Answer : {}", ans);
}
