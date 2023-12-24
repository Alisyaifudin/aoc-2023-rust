use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum Error<'a> {
    NoArg(&'a str),
    Io(io::Error),
    InvalidArg(&'a str),
}

fn main() -> Result<(), Error<'static>> {
    let root_dir = env::current_dir().expect("Failed to get current working directory");
    let args: Vec<String> = env::args().collect();
    // let file_path = root_dir.join("src").join("sample-1.txt");
    let file_path = root_dir.join("src").join("input.txt");
    let mut line_reader = Line::new(file_path).map_err(Error::Io)?;

    let func = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "1" => part_one,
            "2" => part_two,
            _ => {
                return Err(Error::InvalidArg("Invalid argument. Use '1' or '2'"));
            }
        },
        None => {
            return Err(Error::NoArg("Missing argument. Use '1' or '2'"));
        }
    };

    let mut total = 0;
    let mut counter: usize = 1;
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                let subtotal = func(line);
                println!("{}: {}", counter, subtotal);
                total += subtotal;
                counter += 1;
            }
            Ok(None) => {
                // End of file reached
                break;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(Error::Io(e));
            }
        }
    }
    println!("Total = {}", total);

    Ok(())
}
// #################################################################################
// #################################################################################
// #################################################################################
// reading stuff, DO NOT CHANGE (😉)
struct Line {
    reader: BufReader<File>,
}

impl Line {
    fn new(file_path: PathBuf) -> io::Result<Self> {
        let file = File::open(file_path).expect(&*format!("File not found"));
        let reader = BufReader::new(file);
        Ok(Line { reader })
    }

    fn read(&mut self) -> io::Result<Option<String>> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Ok(None), // End of file
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop(); // Remove the trailing newline character
                }
                Ok(Some(line))
            }
            Err(e) => Err(e),
        }
    }
}
// #################################################################################

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Cond {
    Unknown,
    Broken,
    Working,
}

impl std::fmt::Display for Cond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cond::Broken => write!(f, "#"),
            Cond::Working => write!(f, "."),
            Cond::Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq)]
struct State {
    puzzle: Vec<Cond>,
    records: Vec<usize>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.puzzle == other.puzzle && self.records == other.records
    }
}

#[derive(Debug, Clone)]
struct Node {
    state: State,
    vals: Vec<Cond>,
}

impl Node {
    fn new(puzzle: Vec<Cond>, records: Vec<usize>) -> Self {
        Self {
            state: State { puzzle, records },
            vals: Vec::new(),
        }
    }
    fn from(vals: Vec<Cond>, puzzle: Vec<Cond>, records: Vec<usize>) -> Self {
        Self {
            state: State { puzzle, records },
            vals,
        }
    }
}

fn simplified(puzzle: Vec<Cond>) -> Vec<Cond> {
    let mut simple = puzzle.clone();

    // strip Cond::Working at back, left only one
    let mut exist = false;
    for i in (0..simple.len()).rev() {
        if simple[i] == Cond::Working {
            simple.remove(i);
            exist = true;
        } else {
            break;
        }
    }
    if exist {
        simple.push(Cond::Working);
    }

    // strip Cond::Working at front
    simple.reverse();
    // strip Cond::Working at back
    let mut exist = false;
    for i in (0..simple.len()).rev() {
        if simple[i] == Cond::Working {
            simple.remove(i);
            exist = true;
        } else {
            break;
        }
    }
    if exist {
        simple.push(Cond::Working);
    }
    simple.reverse();
    // strip multiple Cond::Working in the middle
    for i in (0..simple.len()).rev() {
        if simple[i] == Cond::Working && i < simple.len() - 1 && simple[i + 1] == Cond::Working {
            simple.remove(i);
        }
    }
    simple
}

#[allow(dead_code)]
fn print_puzzle(puzzle: &Vec<Cond>) {
    for cond in puzzle {
        print!("{}", cond);
    }
    println!();
}


fn calc_combination(node: &Node, remaining: usize, cache: &mut HashMap<State, usize>) -> usize {
    if let Some(n) = cache.get(&node.state) {
        if node.vals.len() > 0 && node.vals.last().unwrap() == &Cond::Working {
            return n.clone();
        }
    }
    if node.state.records.iter().sum::<usize>() == 0 {
        return calc_continue(node);
    }
    let mut counter = 0;
    let (left, right) = step(node, remaining);
    if let Some(left) = left {
        let comb = calc_combination(&left.0, left.1, cache);
        cache.insert(left.0.state, comb);
        counter += comb;
    }
    if let Some(right) = right {
        let comb = calc_combination(&right.0, right.1, cache);
        cache.insert(right.0.state, comb);
        counter += comb;
    }
    counter
}

fn calc_continue(node: &Node) -> usize {
    let mut puzzle = node.state.puzzle.clone();
    let mut vals = node.vals.clone();
    loop {
        if let Some(p) = puzzle.pop() {
            if p == Cond::Broken {
                return 0;
            }
            vals.push(Cond::Working);
        } else {
            vals.reverse();
            // print_puzzle(&vals);
            return 1;
        }
    }
}

fn step(node: &Node, remaining: usize) -> (Option<(Node, usize)>, Option<(Node, usize)>) {
    if let Some(p) = node.state.puzzle.last() {
        match p {
            Cond::Broken => {
                let left = is_broken(node, remaining);
                return (left, None);
            }
            Cond::Working => {
                let right = is_working(node, remaining);
                return (None, right);
            }
            Cond::Unknown => {
                let left = if remaining == 0 {
                    None
                } else {
                    is_broken(node, remaining - 1)
                };
                let right = is_working(node, remaining);
                return (left, right);
            }
        }
    } else {
        (None, None)
    }
}

fn is_broken(node: &Node, remaining: usize) -> Option<(Node, usize)> {
    if node.state.puzzle.len() < remaining
        || node.state.records.len() == 0
        || node.state.records.last().unwrap() == &0
    {
        return None;
    }
    let puzzle = node.state.puzzle.as_slice()[..node.state.puzzle.len() - 1].to_vec();

    let mut vals = node.vals.clone();
    vals.push(Cond::Broken);
    let mut records = node.state.records.clone();
    records[node.state.records.len() - 1] -= 1;
    let new_node = Node::from(vals, puzzle, records);
    return Some((new_node, remaining));
}

fn is_working(node: &Node, remaining: usize) -> Option<(Node, usize)> {
    if node.state.puzzle.len() < remaining
    {
        return None;
    }
    let mut records = node.state.records.clone();
    
    if node.vals.len() > 0 && node.vals.last().unwrap() == &Cond::Broken {
        if node.state.records.len() > 0 && node.state.records.last().unwrap() > &0 {
            return None;
        } else {
            records.pop();
        }
    }

    let puzzle = node.state.puzzle.as_slice()[..node.state.puzzle.len() - 1].to_vec();
    let mut vals = node.vals.clone();
    vals.push(Cond::Working);
    let new_node = Node::from(vals, puzzle, records);
    return Some((new_node, remaining));
}

fn part_one(line: String) -> usize {
    let splitted = line.split_whitespace().collect::<Vec<_>>();
    let records = splitted[1]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let puzzle = splitted[0]
        .chars()
        .map(|s| match s {
            '.' => Cond::Working,
            '#' => Cond::Broken,
            '?' => Cond::Unknown,
            _ => unreachable!(""),
        })
        .collect::<Vec<_>>();

    let puzzle = simplified(puzzle);
    // print_puzzle(&puzzle);
    let remaining =
        records.iter().sum::<usize>() - puzzle.iter().filter(|&p| p == &Cond::Broken).count();
    let head = Node::new(puzzle, records);
    let mut caches = HashMap::new();
    let comb = calc_combination(&head, remaining, &mut caches);
    comb
}

fn part_two(line: String) -> usize {
    let splitted = line.split_whitespace().collect::<Vec<_>>();
    let ori_records = splitted[1]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let ori_puzzle = splitted[0]
        .chars()
        .map(|s| match s {
            '.' => Cond::Working,
            '#' => Cond::Broken,
            '?' => Cond::Unknown,
            _ => unreachable!(""),
        })
        .collect::<Vec<_>>();
    let mut puzzle = ori_puzzle.clone();
    let mut records = ori_records.clone();
    for _ in 1..5 {
        records.extend(ori_records.clone());
        puzzle.push(Cond::Unknown);
        puzzle.extend(ori_puzzle.clone());
    }
    
    puzzle = simplified(puzzle);
    // print_puzzle(&puzzle);
    let remaining =
        records.iter().sum::<usize>() - puzzle.iter().filter(|&p| p == &Cond::Broken).count();
    let head = Node::new(puzzle, records);
    let mut caches = HashMap::new();
    let comb = calc_combination(&head, remaining, &mut caches);
    comb
}