use std::cmp::Ordering::{self};
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

    // to test with sample data
    // let file_path = root_dir.join("src").join("sample-1.txt");
    let file_path = root_dir.join("src").join("input.txt");

    // Create a Line instance from a file
    let line_reader = Line::new(file_path).map_err(Error::Io)?;
    match args.get(1) {
        None => return Err(Error::NoArg("Missing argument. Use '1' or '2'")),
        Some(arg) => {
            match arg.as_str() {
                "1" => part_one(line_reader)?,
                "2" => part_two(line_reader)?,
                _ => return Err(Error::InvalidArg("Invalid argument. Use '1' or '2'")),
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
            Ok(_) => Ok(Some(line)),
            Err(e) => Err(e),
        }
    }
}

// ##################################################################################

fn determine_type(cards: &str) -> Type {
    let sets = create_sets(cards);
    if sets.len() == 1 {
        return Type::FiveOfAKind;
    }
    if sets.len() == 2 {
        if sets[0].len == 1 {
            return Type::FourOfAKind;
        }
        if sets[0].len == 2 {
            return Type::FullHouse;
        }
    }
    if sets.len() == 3 {
        if sets[1].len == 1 {
            return Type::ThreeOfAKind;
        }
        if sets[1].len == 2 {
            return Type::TwoPair;
        }
    }
    if sets.len() == 4 {
        return Type::OnePair;
    }
    Type::HighCard
}
fn create_sets(cards: &str) -> Vec<Set> {
    let mut sets: Vec<Set> = Vec::new();
    'outer: for card in cards.chars() {
        for set in sets.iter_mut() {
            if set.card == card {
                set.update();
                continue 'outer;
            }
        }
        sets.push(Set::new(card));
    }
    sets.sort_by_key(|s| s.len);
    sets
}

// enum
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Set {
    card: char,
    len: usize,
}

impl Set {
    fn new(card: char) -> Self {
        Set { card, len: 1 }
    }
    fn update(&mut self) {
        self.len += 1;
    }
}

// ########################################################################################3
// part one

fn read_file_one(mut line_reader: Line) -> Result<Vec<HandOne>, Error<'static>> {
    let mut hands = Vec::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                let splitted = line.split_ascii_whitespace().collect::<Vec<_>>();
                if splitted.len() != 2 {
                    panic!("The line's len() is not 2!");
                }
                let cards = splitted[0];
                let bid = splitted[1].parse::<usize>().expect("Failed to parse");
                hands.push(HandOne::new(cards, bid));
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
    Ok(hands)
}

#[derive(Debug, Eq, Ord)]
struct HandOne {
    typ: Type,
    cards: String,
    bid: usize,
}

impl HandOne {
    fn new(cards: &str, bid: usize) -> Self {
        HandOne {
            cards: cards.into(),
            bid,
            typ: determine_type(cards),
        }
    }
    fn value(card: &char) -> usize {
        match *card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for HandOne {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
            && self
                .cards
                .chars()
                .zip(other.cards.chars())
                .all(|(ch1, ch2)| HandOne::value(&ch1) == HandOne::value(&ch2))
    }
}

impl PartialOrd for HandOne {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.typ.partial_cmp(&other.typ) {
            Some(Ordering::Equal) => {
                for (ch1, ch2) in self.cards.chars().zip(other.cards.chars()) {
                    let v1 = HandOne::value(&ch1);
                    let v2 = HandOne::value(&ch2);
                    if v1 != v2 {
                        return Some(v1.cmp(&v2));
                    }
                }
                // Equal
                return Some(Ordering::Equal);
            }
            result => result,
        }
    }
}

fn part_one(line_reader: Line) -> Result<(), Error<'static>> {
    let mut hands = read_file_one(line_reader)?;
    hands.sort();
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        println!("Rank: {} {:?}", i + 1, hand);
        total += hand.bid * (i + 1);
    }
    println!("total = {}", total);
    Ok(())
}

// ########################################################################################3
// part two
fn read_file_two(mut line_reader: Line) -> Result<Vec<HandTwo>, Error<'static>> {
    let mut hands = Vec::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                let splitted = line.split_ascii_whitespace().collect::<Vec<_>>();
                if splitted.len() != 2 {
                    panic!("The line's len() is not 2!");
                }
                let cards = splitted[0];
                let bid = splitted[1].parse::<usize>().expect("Failed to parse");
                hands.push(HandTwo::new(cards, bid));
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
    Ok(hands)
}

// const CARDS: [char; 12] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

struct Combination {
    current: Vec<usize>,
    len: usize,
    overflow: bool,
}

impl Combination {
    fn new(len: usize) -> Self {
        let overflow = len == 0;
        Combination {
            current: vec![0; len],
            len,
            overflow,
        }
    }
}

impl Iterator for Combination {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();
        let mut overflow = true;
        if !self.overflow {
            for i in 0..self.len {
                let added = self.current[i] + 1;
                if added > 12 {
                    self.current[i] = 0;
                    continue;
                }
                self.current[i] = added;
                overflow = false;
                break;
            }
            self.overflow = overflow;
            Some(current)
        } else {
            None
        }
    }
}

fn determine_type_further(cards: &str) -> Type {
    let mut current_type = determine_type(cards);
    let mut cards_char = cards.chars().collect::<Vec<_>>();
    let indices = cards
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == 'J')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let len = indices.len();
    let combination = Combination::new(len);
    for com in combination {
        for (index, new_char) in indices.iter().zip(com.iter()) {
            cards_char[*index] = HandTwo::inv_value(new_char);
        }
        let new_cards = cards_char.iter().collect::<String>();
        let new_type = determine_type(&new_cards);
        if new_type > current_type {
            current_type = new_type;
        }
        if current_type == Type::FiveOfAKind {
            break;
        }
    }
    current_type
}

#[derive(Debug, Eq, Ord)]
struct HandTwo {
    typ: Type,
    cards: String,
    bid: usize,
}

impl HandTwo {
    fn new(cards: &str, bid: usize) -> Self {
        HandTwo {
            cards: cards.into(),
            bid,
            typ: determine_type_further(cards),
        }
    }

    fn value(card: &char) -> usize {
        match *card {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            'J' => 0,
            _ => unreachable!(),
        }
    }
    fn inv_value(value: &usize) -> char {
        match *value {
            12 => 'A',
            11 => 'K',
            10 => 'Q',
            9 => 'T',
            8 => '9',
            7 => '8',
            6 => '7',
            5 => '6',
            4 => '5',
            3 => '4',
            2 => '3',
            1 => '2',
            0 => 'J',
            _ => unreachable!(),
        }
    }
}

impl PartialEq for HandTwo {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
            && self
                .cards
                .chars()
                .zip(other.cards.chars())
                .all(|(ch1, ch2)| HandTwo::value(&ch1) == HandTwo::value(&ch2))
    }
}

impl PartialOrd for HandTwo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.typ.partial_cmp(&other.typ) {
            Some(Ordering::Equal) => {
                for (ch1, ch2) in self.cards.chars().zip(other.cards.chars()) {
                    let v1 = HandTwo::value(&ch1);
                    let v2 = HandTwo::value(&ch2);
                    if v1 != v2 {
                        return Some(v1.cmp(&v2));
                    }
                }
                // Equal
                return Some(Ordering::Equal);
            }
            result => result,
        }
    }
}

fn part_two(line_reader: Line) -> Result<(), Error<'static>> {
    let mut hands = read_file_two(line_reader)?;
    hands.sort();
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        println!("Rank: {} {:?}", i + 1, hand);
        total += hand.bid * (i + 1);
    }
    println!("total = {}", total);
    Ok(())
}
