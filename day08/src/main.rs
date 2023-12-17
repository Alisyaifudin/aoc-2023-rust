use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum Invalid<'a> {
    Arg(&'a str),
}

#[derive(Debug)]
enum Error<'a> {
    NoArg(&'a str),
    Io(io::Error),
    Inv(Invalid<'a>),
}

fn main() -> Result<(), Error<'static>> {
    let root_dir = env::current_dir().expect("Failed to get current working directory");
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => return Err(Error::NoArg("Missing argument. Use '1' or '2'")),
        Some(arg) => {
            match arg.as_str() {
                "1" => {
                    // let file_path = root_dir.join("src").join("sample-2.txt");
                    let file_path = root_dir.join("src").join("input.txt");
                    let line_reader = Line::new(file_path).map_err(Error::Io)?;
                    part_one(line_reader)?
                }
                "2" => {
                    // let file_path = root_dir.join("src").join("sample-3.txt");
                    let file_path = root_dir.join("src").join("input.txt");
                    let line_reader = Line::new(file_path).map_err(Error::Io)?;
                    part_two(line_reader)?
                }
                _ => return Err(Error::Inv(Invalid::Arg("Invalid argument. Use '1' or '2'"))),
            };
        }
    };

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
#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Left,
    Right,
}

// struct Node {
//     value: String,
//     left: Option<Box<Node>>,
//     right: Option<Box<Node>>,
// }

#[derive(Clone)]
struct Node {
    value: u16,
    string: String,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn convert(value: &str) -> u16 {
        let cs = value
            .chars()
            .map(|c| ((c as u8) - ('A' as u8)) as u16)
            .collect::<Vec<u16>>();
        let val = cs
            .into_iter()
            .enumerate()
            .reduce(|(_, acc), (i, e)| (i, acc + e * 26_u16.pow(i as u32)))
            .map(|a| a.1)
            .expect("Cannot convert it?");
        val
    }
    fn new(value: &str) -> Self {
        Node {
            value: Node::convert(value),
            string: value.into(),
            left: None,
            right: None,
        }
    }
    fn with_children(value: &str, left: usize, right: usize) -> Self {
        Node {
            value: Node::convert(value),
            string: value.into(),
            left: Some(left),
            right: Some(right),
        }
    }
    fn add_left(&mut self, index: usize) {
        self.left = Some(index);
    }
    fn add_right(&mut self, index: usize) {
        self.right = Some(index);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ value: {} }}", self.string)
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

// function

// fn get_or_create(graph: Graph, value: &str) ->

fn read_file(
    mut line_reader: Line,
    graph: &mut impl Graph,
) -> Result<Vec<Instruction>, Error<'static>> {
    // read first line
    let first_line = line_reader.read().map_err(Error::Io)?;
    let instructions = match first_line {
        Some(line) => line
            .chars()
            .map(|c| {
                if c == 'L' {
                    Instruction::Left
                } else {
                    Instruction::Right
                }
            })
            .collect::<Vec<_>>(),
        None => panic!("No first line found"),
    };

    let _ = line_reader.read(); // skip second line

    // let mut graph = GraphTwo::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                let splitted = line.split("=").map(|l| l.trim()).collect::<Vec<_>>();
                if splitted.len() != 2 {
                    panic!("Not 2 parts!")
                }
                let head = splitted[0];
                let len = splitted[1].len();
                let directions = splitted[1][1..len - 1]
                    .split(",")
                    .map(|s| s.trim())
                    .collect::<Vec<_>>();
                if directions.len() != 2 {
                    panic!("No directions??")
                }
                let [left, right] = [directions[0], directions[1]];
                graph.add(head, left, right);
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
    Ok(instructions)
}

// part one
struct GraphOne {
    nodes: Vec<Node>,
}

trait Graph {
    fn get(&self, value: &str) -> Option<&Node>;
    fn get_index(&self, value: &str) -> Option<usize>;
    fn add(&mut self, head: &str, left: &str, right: &str);
}

impl GraphOne {
    fn new() -> Self {
        GraphOne { nodes: Vec::new() }
    }
}

impl Graph for GraphOne {
    fn get(&self, value: &str) -> Option<&Node> {
        self.nodes.iter().find(|&n| n.value == Node::convert(value))
    }
    fn get_index(&self, value: &str) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_, n)| n.value == Node::convert(value))
            .map(|(i, _)| i)
    }
    fn add(&mut self, head: &str, left: &str, right: &str) {
        let left_index = if let Some(i) = self.get_index(left) {
            i
        } else {
            self.nodes.push(Node::new(left));
            self.nodes.len() - 1
        };
        let right_index = if let Some(i) = self.get_index(right) {
            i
        } else {
            self.nodes.push(Node::new(right));
            self.nodes.len() - 1
        };
        match self.get_index(head) {
            None => {
                let head_node = Node::with_children(head, left_index, right_index);
                self.nodes.push(head_node);
            }
            Some(head_index) => {
                self.nodes[head_index].add_left(left_index);
                self.nodes[head_index].add_right(right_index);
            }
        }
    }
}
fn part_one(line_reader: Line) -> Result<(), Error<'static>> {
    let mut graph = GraphOne::new();
    let instructions = read_file(line_reader, &mut graph)?;
    let mut current = graph.get("AAA").expect("AAA not found");
    let end = graph.get("ZZZ").expect("ZZZ not found");
    let mut counter: u64 = 0;
    'outer: loop {
        for instruction in &instructions {
            if current == end {
                break 'outer;
            }
            match instruction {
                Instruction::Left => {
                    current = &graph.nodes[current.left.expect("No left turn 😵‍💫")]
                }
                Instruction::Right => {
                    current = &graph.nodes[current.right.expect("No right turn 😵‍💫")]
                }
            }
            counter += 1;
        }
    }
    println!("total: {}", counter);

    Ok(())
}

// #################################################################################
// part two
struct GraphTwo {
    nodes: Vec<Node>,
    starts: Vec<u16>,
    ends: Vec<u16>,
}

impl GraphTwo {
    fn new() -> Self {
        GraphTwo {
            nodes: Vec::new(),
            starts: Vec::new(),
            ends: Vec::new(),
        }
    }
    fn push(&mut self, value: &str) {
        if value.ends_with("A") {
            self.starts.push(Node::convert(value));
        }
        if value.ends_with("Z") {
            self.ends.push(Node::convert(value));
        }
    }
}

impl Graph for GraphTwo {
    fn get(&self, value: &str) -> Option<&Node> {
        self.nodes.iter().find(|&n| n.value == Node::convert(value))
    }
    fn get_index(&self, value: &str) -> Option<usize> {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_, n)| n.value == Node::convert(value))
            .map(|(i, _)| i)
    }

    fn add(&mut self, head: &str, left: &str, right: &str) {
        let left_index = if let Some(i) = self.get_index(left) {
            i
        } else {
            self.nodes.push(Node::new(left));
            self.push(left);
            self.nodes.len() - 1
        };
        let right_index = if let Some(i) = self.get_index(right) {
            i
        } else {
            self.nodes.push(Node::new(right));
            self.push(right);
            self.nodes.len() - 1
        };
        match self.get_index(head) {
            None => {
                let head_node = Node::with_children(head, left_index, right_index);
                self.push(head);
                self.nodes.push(head_node);
            }
            Some(head_index) => {
                self.nodes[head_index].add_left(left_index);
                self.nodes[head_index].add_right(right_index);
            }
        }
    }
}

struct List {
    initials: HashSet<usize>,
    boundary: usize,
    period: usize,
    contents: HashSet<usize>,
}

impl List {
    /*
    initials is list of Zs position before entering the cycle
    contents is list of Zs position after entering the cycle
    boundary is seperating the initial to in cycle
    period is the period of the cycle. There must be cycle because
    the instruction is repetitive. The cycle must be multiple
    of the length of the movements.
    */
    fn new(period: usize, boundary: usize) -> Self {
        List {
            initials: HashSet::new(),
            period,
            boundary,
            contents: HashSet::new(),
        }
    }
    fn add_initial(&mut self, value: usize) {
        self.initials.insert(value);
    }
    fn add_content(&mut self, value: usize) {
        self.contents.insert(value);
    }
    fn find(&self, step: usize) -> bool {
        if step < self.initials.len() {
            return self.initials.get(&step).is_some();
        }
        return self
            .contents
            .get(&((step - &self.boundary) % &self.period))
            .is_some();
    }
    fn get(&self) -> (Vec<usize>, Vec<usize>, usize, usize) {
        // true if in contents
        let initials = self.initials.iter().cloned().collect::<Vec<_>>();
        let contents = self.contents.iter().cloned().collect::<Vec<_>>();
        (
            initials,
            contents,
            self.period.clone(),
            self.boundary.clone(),
        )
    }
}

impl std::fmt::Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "initials:\t{:?}\ncontents:\t{:?}\nperiod:  \t{}\nboundary:\t{}",
            self.initials, self.contents, self.period, self.boundary
        )
    }
}

fn part_two(line_reader: Line) -> Result<(), Error<'static>> {
    let mut graph = GraphTwo::new();
    let instructions = read_file(line_reader, &mut graph)?;

    let nodes = graph
        .starts
        .into_iter()
        .map(|s| {
            graph
                .nodes
                .iter()
                .find(|&n| n.value == s)
                .expect("not start found?")
        })
        .collect::<Vec<_>>();
    let mut counter = 0;
    let len = instructions.len();
    let mut lists = Vec::new();
    // creating lists
    for node in nodes {
        let mut current = &node.clone();
        let mut sequences = Vec::new();
        let mut traces = Vec::new();
        println!("\n===== {} Position", node.string);
        'outer: loop {
            traces.push(current.clone());
            for instruction in &instructions {
                match instruction {
                    Instruction::Left => {
                        current = &graph.nodes[current.left.expect("No left turn 😵‍💫")];
                    }
                    Instruction::Right => {
                        current = &graph.nodes[current.right.expect("No right turn 😵‍💫")];
                    }
                }
                traces.push(current.clone());
                counter += 1;
            }
            sequences.push(traces.clone());
            print!("[{}] -> ", traces[0].string);
            for (i, seq) in traces.iter().enumerate() {
                if i == instructions.len() || i == 0 {
                    continue;
                }
                if seq.string.ends_with("Z") {
                    print!("({}) {} -> ", i + counter - instructions.len(), seq.string);
                }
            }
            print!("[{}] {}\n", traces.last().unwrap().string, counter);

            let seq_index = sequences
                .iter()
                .enumerate()
                .find(|(_, p)| p[0] == *current)
                .map(|(i, _)| i);
            if let Some(index) = seq_index {
                traces.clear();
                counter = 0;
                let mut list = List::new(len * (sequences.len() - index), index * len);
                for (i, seq) in sequences.iter().enumerate() {
                    if i < index {
                        for (j, s) in seq.iter().enumerate() {
                            if j == seq.len() - 1 {
                                continue;
                            }
                            if s.string.ends_with("Z") {
                                list.add_initial(j + i * len);
                            }
                        }
                    } else {
                        for (j, s) in seq.iter().enumerate() {
                            if j == seq.len() - 1 {
                                continue;
                            }
                            if s.string.ends_with("Z") {
                                list.add_content(j + (i - index) * len);
                            }
                        }
                    }
                }
                println!("{:?}", list);
                lists.push(list);
                break 'outer;
            }
            traces.clear();
        }
    }

    // executing...
    let (initials, contents, period, boundary) = lists[0].get();
    let mut complete = true;
    let mut loc = 0;
    // if we find all zs in the initials moves
    for t in &initials {
        loc = t.clone();
        for list in &lists {
            if !list.find(loc) {
                complete = false;
                break;
            }
        }
    }
    if complete && initials.len() > 0 {
        println!("found! {}", loc);
        return Ok(());
    }
    // most likely, the zs in the cycle moves
    let mut cycle: usize = 0;
    println!();
    'outer: loop {
        let mut complete = true;
        'inner: for t in &contents {
            loc = t.clone() + cycle * period + boundary;
            for list in &lists {
                if !list.find(loc) {
                    complete = false;
                    continue 'inner;
                }
            }
            if complete {
                break 'outer;
            }
        }
        cycle += 1;
        if cycle % 100_000 == 0 {
            print!("\rCounter: {}, cycle: {}", loc, cycle);
        }
    }
    println!("\nfound! {}", loc);
    Ok(())
}
