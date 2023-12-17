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
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                let sequence = line
                    .split_ascii_whitespace()
                    .map(|l| l.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                let prediction = func(&sequence);
                // println!("{:?}", sequence);
                println!("prediction = {}", prediction);
                println!();
                total += prediction;
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

fn part_one(sequence: &Vec<i32>) -> i32 {
    let mut sequences = vec![sequence.clone()];
    let mut next_seq = Vec::new();
    println!("{:?}", sequences[0]);
    for _ in 1..sequence.len() {
        let current_seq = sequences.last().unwrap().iter().rev().collect::<Vec<_>>();
        for seq in current_seq.windows(2) {
            next_seq.push(seq[0] - seq[1])
        }
        next_seq.reverse();
        println!("{:?}", next_seq);
        if next_seq.iter().all(|&s| s == next_seq[0]) {
            sequences.push(std::mem::take(&mut next_seq));
            break;
        }
        sequences.push(std::mem::take(&mut next_seq));
    }
    let res = sequences
        .iter()
        .map(|s| s.last().unwrap())
        .cloned()
        .reduce(|acc, e| acc + e)
        .unwrap();

    res
}

// part two
fn part_two(sequence: &Vec<i32>) -> i32 {
    let mut sequences = vec![sequence.clone()];
    let mut next_seq = Vec::new();
    for _ in 1..sequence.len() {
        let current_seq = sequences.last().unwrap().iter().rev().collect::<Vec<_>>();
        for seq in current_seq.windows(2) {
            next_seq.push(seq[0] - seq[1])
        }
        next_seq.reverse();
        if next_seq.iter().all(|&s| s == next_seq[0]) {
            sequences.push(std::mem::take(&mut next_seq));
            break;
        }
        sequences.push(std::mem::take(&mut next_seq));
    }
    let mut curr = 0;
    for seq in sequences.iter_mut().rev() {
        let first = seq[0];
        curr = first - curr;
        seq.insert(0, curr);
    }
    for seq in &sequences {
        println!("{:?}", seq);
    }
    sequences[0][0]
}
