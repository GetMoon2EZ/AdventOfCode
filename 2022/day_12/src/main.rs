use std::fs::{File};
use std::io::{BufReader, BufRead, Lines};
use clap::{Parser};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::dijkstra;
use std::cmp::min;

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

fn parse_input(filename: &str) -> (Graph<u8, u8>, NodeIndex, NodeIndex) {
    let mut map: Vec<Vec<NodeIndex>> = Vec::new();
    let mut graph = Graph::<u8, u8>::new();
    let mut start = Default::default();
    let mut end = Default::default();

    let lines = read_lines(filename);
    for line in lines {
        let mut row = Vec::new();
        if let Ok(s) = line {
            for c in s.chars() {
                let character = match c {
                    'S' => { 'a' },
                    'E' => { 'z'},
                    c => { c },
                };
                let node = graph.add_node(character as u8);
                if c == 'S' {
                    start = node;
                }
                if c == 'E' {
                    end = node;
                }
                row.push(node);
            }
        }
        map.push(row);
    }
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let current_weight = graph.node_weight(map[y][x]).unwrap() + 1;
            if y != 0 && *graph.node_weight(map[y - 1][x]).unwrap() <= current_weight {
                    graph.add_edge(map[y][x], map[y - 1][x], 1);
            }
            if y + 1 != map.len() && *graph.node_weight(map[y + 1][x]).unwrap() <= current_weight {
                    graph.add_edge(map[y][x], map[y + 1][x], 1);
            }
            if x != 0 && *graph.node_weight(map[y][x - 1]).unwrap() <= current_weight {
                graph.add_edge(map[y][x], map[y][x - 1], 1);
            }
            if x + 1 != map[y].len() && *graph.node_weight(map[y][x + 1]).unwrap() <= current_weight {
                graph.add_edge(map[y][x], map[y][x + 1], 1);
            }
        }
    }
    return (graph, start, end);
}

fn solve_problem_1(filename: &str) {
    let (graph, start, end) = parse_input(filename);

    let res = dijkstra(&graph, start, Some(end), |_| 1);
    let ans = res.get(&end).unwrap();
    println!("Answer: {:?}", ans);
}

fn solve_problem_2(filename: &str) {
    let (graph, _, end) = parse_input(filename);
    let mut ans = std::i32::MAX;
    for inode in 0..graph.node_count() {
        let node_index = NodeIndex::new(inode);
        if *graph.node_weight(node_index).unwrap() == 'a' as u8 {
            let res = dijkstra(&graph, node_index, Some(end), |_| 1);
            let val = match res.get(&end) {
                Some(&val) => { val },
                None => { std::i32::MAX },
            };
            ans = min(ans, val);
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
