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

    // to test with sample data
    // let file_path = root_dir.join("src").join("sample-1.txt");
    let file_path = root_dir.join("src").join("input.txt");

    // Create a Line instance from a file
    let mut line_reader = Line::new(file_path).map_err(Error::Io)?;

    let mut total: u32 = 0;

    let func = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "1" => part_one,
            "2" => part_two,
            _ => {
                return Err(Error::Inv(Invalid::Arg("Invalid argument. Use '1' or '2'")));
            }
        },
        None => {
            return Err(Error::NoArg("Missing argument. Use '1' or '2'"));
        }
    };
    let mut lines = Lines::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                lines.update(Some(line));
                let subtotal = func(&lines);
                total += subtotal;
            }
            Ok(None) => {
                // End of file reached
                // one more time...
                lines.update(None);
                let subtotal = func(&lines);
                total += subtotal;
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
enum Char {
    Digit(u8),
    Important,
    Other,
}

// struct
struct LinesIterator {
    lines: Lines,
    index: usize,
}

impl Iterator for LinesIterator {
    type Item = (usize, Option<char>, Option<char>, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        let ch_above = self.lines.above.as_ref().and_then(|s| s.chars().nth(index));
        let ch_middle = self
            .lines
            .middle
            .as_ref()
            .and_then(|s| s.chars().nth(index));
        let ch_below = self.lines.below.as_ref().and_then(|s| s.chars().nth(index));

        if ch_above.is_some() || ch_middle.is_some() || ch_below.is_some() {
            self.index += 1;
            Some((index, ch_above, ch_middle, ch_below))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Lines {
    above: Option<String>,
    middle: Option<String>,
    below: Option<String>,
}

impl Lines {
    fn new() -> Self {
        Lines {
            above: None,
            middle: None,
            below: None,
        }
    }
    fn iter(&self) -> LinesIterator {
        LinesIterator {
            lines: self.clone(),
            index: 0,
        }
    }
    fn update(&mut self, line: Option<String>) {
        self.above = self.middle.take();
        self.middle = self.below.take();
        self.below = line;
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    start: usize,
    len: usize,
}

impl Number {
    fn new<'a>(digit: u8, start: usize) -> Self {
        Number {
            value: digit as u32,
            start,
            len: 1,
        }
    }
    fn add(mut self, digit: u8) -> Self {
        self.value = self.value * 10 + digit as u32;
        self.len += 1;
        self
    }
    fn end(&self) -> usize {
        self.len + self.start
    }
}

struct Container<T, const SIZE: usize> {
    temps: [Option<T>; SIZE],
    contents: Vec<T>,
}

impl Container<Number, 1> {
    fn new() -> Self {
        Container {
            temps: [None],
            contents: Vec::new(),
        }
    }
    fn update(&mut self, start: usize, digit: u8) {
        if self.temps[0].is_none() {
            self.temps[0] = Some(Number::new(digit, start));
        } else if let Some(num) = self.temps[0].take() {
            self.temps[0] = Some(num.add(digit));
        }
    }
    fn clear(&mut self) {
        if let Some(it) = self.temps[0].take() {
            self.contents.push(it);
        }
    }
}

impl Container<Number, 3> {
    fn new() -> Self {
        Container {
            temps: [None, None, None],
            contents: Vec::new(),
        }
    }
    fn update(&mut self, index: usize, start: usize, digit: u8) {
        self.is_out_of_bound(index);
        if self.temps[index].is_none() {
            self.temps[index] = Some(Number::new(digit, start));
        } else if let Some(num) = self.temps[index].take() {
            self.temps[index] = Some(num.add(digit));
        }
    }
    fn clear(&mut self, index: usize) {
        self.is_out_of_bound(index);
        if let Some(it) = self.temps[index].take() {
            self.contents.push(it);
        }
    }
    fn is_out_of_bound(&self, index: usize) {
        if index > 2 {
            panic!("Symbol index must be between 0, 1, or 2");
        }
    }
}

#[derive(Debug, Clone)]
struct Symbol {
    start: usize,
    len: usize,
}

impl Symbol {
    fn new(start: usize) -> Self {
        Symbol { len: 1, start }
    }
    fn add(mut self) -> Self {
        self.len += 1;
        self
    }
    fn end(&self) -> usize {
        self.len + self.start
    }
}

impl Container<Symbol, 3> {
    fn new() -> Self {
        Container {
            temps: [None, None, None],
            contents: Vec::new(),
        }
    }
    fn update(&mut self, index: usize, start: usize) {
        self.is_out_of_bound(index);
        if self.temps[index].is_none() {
            self.temps[index] = Some(Symbol::new(start));
        } else if let Some(sym) = self.temps[index].take() {
            self.temps[index] = Some(sym.add());
        }
    }
    fn clear(&mut self, index: usize) {
        self.is_out_of_bound(index);
        if let Some(it) = self.temps[index].take() {
            self.contents.push(it);
        }
    }
    fn is_out_of_bound(&self, index: usize) {
        if index > 2 {
            panic!("Symbol index must be between 0, 1, or 2");
        }
    }
}

type Gear = Symbol;
impl Container<Gear, 1> {
    fn new() -> Self {
        Container {
            temps: [None],
            contents: Vec::new(),
        }
    }
    fn update(&mut self, start: usize) {
        if self.temps[0].is_none() {
            self.temps[0] = Some(Symbol::new(start));
        } else if let Some(gear) = self.temps[0].take() {
            self.temps[0] = Some(gear.add());
        }
    }
    fn clear(&mut self) {
        if let Some(it) = self.temps[0].take() {
            self.contents.push(it);
        }
    }
}
// ##########################################################################
// main
// part one
fn find_char_part_one(ch: Option<char>) -> Char {
    match ch.map(|c| c.to_ascii_lowercase()) {
        Some('0'..='9') => Char::Digit(ch.unwrap() as u8 - b'0'),
        Some('.') => Char::Other,
        _ => Char::Important,
    }
}

fn part_one(lines: &Lines) -> u32 {
    if lines.middle.is_none() {
        return 0;
    }
    let mut numbers = Container::<Number, 1>::new();
    let mut symbols = Container::<Symbol, 3>::new();
    for (i, ch_above, ch_middle, ch_below) in lines.iter() {
        // above
        match find_char_part_one(ch_above) {
            Char::Important => symbols.update(0, i),
            _ => symbols.clear(0),
        }
        // middle
        match find_char_part_one(ch_middle) {
            Char::Digit(digit) => {
                numbers.update(i, digit);
                symbols.clear(1);
            }
            Char::Important => {
                symbols.update(1, i);
                numbers.clear();
            }
            Char::Other => {
                numbers.clear();
                symbols.clear(1);
            }
        }
        // below
        match find_char_part_one(ch_below) {
            Char::Important => symbols.update(2, i),
            _ => symbols.clear(2),
        }
    }
    // loop over all numbers
    let mut subtotal = 0;
    for num in &numbers.contents {
        for sym in &symbols.contents {
            if sym.end() >= num.start && num.end() >= sym.start {
                subtotal += num.value;
            }
        }
    }
    return subtotal;
}

// part two
fn find_char_part_two(ch: Option<char>) -> Char {
    match ch.map(|c| c.to_ascii_lowercase()) {
        Some('0'..='9') => Char::Digit(ch.unwrap() as u8 - b'0'),
        Some('*') => Char::Important,
        _ => Char::Other,
    }
}

fn part_two(lines: &Lines) -> u32 {
    if lines.middle.is_none() {
        return 0;
    }
    let mut numbers = Container::<Number, 3>::new();
    let mut gears = Container::<Gear, 1>::new();
    for (i, ch_above, ch_middle, ch_below) in lines.iter() {
        // above
        match find_char_part_two(ch_above) {
            Char::Digit(digit) => numbers.update(0, i, digit),
            _ => numbers.clear(0),
        }
        // middle
        match find_char_part_two(ch_middle) {
            Char::Digit(digit) => {
                numbers.update(1, i, digit);
                gears.clear();
            }
            Char::Important => {
                gears.update(i);
                numbers.clear(1);
            }
            Char::Other => {
                numbers.clear(1);
                gears.clear();
            }
        }
        // below
        match find_char_part_two(ch_below) {
            Char::Digit(digit) => numbers.update(2, i, digit),
            _ => numbers.clear(2),
        }
    }
    // loop over all numbers
    let mut subtotal = 0;
    let mut gear_numbers = Vec::new();
    for gear in &gears.contents {
        for num in &numbers.contents {
            if gear.end() >= num.start && num.end() >= gear.start {
                gear_numbers.push(num.value);
            }
        }
        if gear_numbers.len() == 2 {
            subtotal += gear_numbers[0] * gear_numbers[1];
        }
        gear_numbers.clear();
    }
    return subtotal;
}
