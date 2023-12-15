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

#[derive(PartialEq)]
enum Case {
    One,
    Two,
}

fn main() -> Result<(), Error<'static>> {
    let root_dir = env::current_dir().expect("Failed to get current working directory");
    let args: Vec<String> = env::args().collect();

    // to test with sample data
    // let file_path = root_dir.join("src").join("sample-1.txt");
    let file_path = root_dir.join("src").join("input.txt");

    // Create a Line instance from a file
    let mut line_reader = Line::new(file_path).map_err(Error::Io)?;

    let case = args
        .get(1)
        .map(|arg| match arg.as_str() {
            "1" => Ok(Case::One),
            "2" => Ok(Case::Two),
            _ => Err(Error::Inv(Invalid::Arg("Invalid argument. Use '1' or '2'"))),
        })
        .unwrap_or_else(|| Err(Error::NoArg("Missing argument. Use '1' or '2'")))?;
    let mut total = 0;
    let mut bonus = Vec::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => match &case {
                &Case::One => total += part_one(line),
                &Case::Two => total += part_two(line, &mut bonus),
            },
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

    println!("Total: {}", total);

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
            Ok(_) => Ok(Some(line)),
            Err(e) => Err(e),
        }
    }
}

// #################################################################################
// #################################################################################
// #################################################################################

// enum

// struct

// main
// part one
fn part_one(line: String) -> u32 {
    let cards = *line
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("No games!");
    let splitted_cards = cards.split('|').collect::<Vec<&str>>();
    let [winning_cards, your_cards] = [splitted_cards[0], splitted_cards[1]];
    let mut total = 0;
    for winning_card in winning_cards.split_ascii_whitespace() {
        for your_card in your_cards.split_ascii_whitespace() {
            if winning_card == your_card {
                if total == 0 {
                    total = 1;
                } else {
                    total *= 2;
                }
            }
        }
    }
    total
}

fn pop_first<T>(vector: &mut Vec<T>, default: T) -> T {
    if vector.len() == 0 {
        default
    } else {
        vector.remove(0)
    }
}

// part two
fn part_two(line: String, bonus: &mut Vec<usize>) -> u32 {
    let cards = *line
        .split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("No games!");
    let splitted_cards = cards.split('|').collect::<Vec<&str>>();
    let [winning_cards, your_cards] = [splitted_cards[0], splitted_cards[1]];
    let mut matching: usize = 0;
    for winning_card in winning_cards.split_ascii_whitespace() {
        for your_card in your_cards.split_ascii_whitespace() {
            if winning_card == your_card {
                matching += 1;
            }
        }
    }
    let total = 1 + pop_first(bonus, 0);
    for i in 0..matching {
        if let Some(value) = bonus.get_mut(i) {
            *value += total; // Element exists, increment by one
        } else {
            bonus.push(total); // Element doesn't exist, append new element with value 0
        }
    }
    println!("matching: {}, total = {}", matching, total);
    println!("bonus: {:?}", bonus);
    total as u32
}
