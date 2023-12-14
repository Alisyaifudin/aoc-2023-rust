use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
enum Error {
    NoArg,
    InvalidArg,
    Io(io::Error),
}

fn main() -> Result<(), Error> {
    let root_dir = env::current_dir().expect("Failed to get current working directory");
    let args: Vec<String> = env::args().collect();

    // to test with sample data
    // let file_path = root_dir.join("src").join("sample-2.txt");
    let file_path = root_dir.join("src").join("input.txt");
    
    // Create a Line instance from a file
    let mut line_reader = Line::new(file_path).map_err(Error::Io)?;

    let mut total: u32 = 0;
    let func = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "1" => part_one,
            "2" => part_two,
            _ => {
                eprintln!("Invalid argument. Use '1' or '2'");
                return Err(Error::InvalidArg);
            }
        },
        None => {
            eprintln!("Missing argument. Use '1' or '2'");
            return Err(Error::NoArg);
        }
    };
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                total += func(line);
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

    println!("Total: {}", total);

    Ok(())
}

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

fn char_to_digit(c: char) -> Option<u8> {
    if c.is_digit(10) {
        Some(c as u8 - b'0')
    } else {
        None
    }
}

#[allow(dead_code)]
fn part_one(line: String) -> u32 {
    let mut digits: Vec<u8> = Vec::new();
    for ch in line.chars() {
        match char_to_digit(ch) {
            Some(num) => digits.push(num),
            None => (),
        }
    }
    if digits.len() == 0 {
        panic!("There must be number in each line! {}", line)
    }
    let first_digit = digits.first().unwrap();
    let last_digit = digits.last().unwrap();
    return (first_digit * 10 + last_digit) as u32;
}

struct Digit {
    digit: u8,
    buff: Vec<char>,
    spell_out: Vec<char>,
}

impl Digit {
    fn new(digit: u8, spell_out: &str) -> Self {
        Digit {
            digit,
            buff: Vec::new(),
            spell_out: spell_out.chars().collect(),
        }
    }
    fn update(&mut self, ch: char) -> Option<u8> {
        let buff_len = self.buff.len();
        if self.spell_out[buff_len] != ch {
            if buff_len > 0 {
                self.buff = Vec::new();
            }
            // start over
            if self.spell_out[0] == ch {
                self.buff.push(ch);
            }
            return None;
        }
        if self.spell_out.len() > buff_len + 1 {
            self.buff.push(ch);
            return None;
        }
        self.buff = Vec::new();
        return Some(self.digit);
    }
}

fn create_digits() -> Vec<Digit> {
    let mut digits = Vec::new();
    digits.push(Digit::new(0, "zero"));
    digits.push(Digit::new(1, "one"));
    digits.push(Digit::new(2, "two"));
    digits.push(Digit::new(3, "three"));
    digits.push(Digit::new(4, "four"));
    digits.push(Digit::new(5, "five"));
    digits.push(Digit::new(6, "six"));
    digits.push(Digit::new(7, "seven"));
    digits.push(Digit::new(8, "eight"));
    digits.push(Digit::new(9, "nine"));
    return digits;
}

#[allow(dead_code)]
fn part_two(line: String) -> u32 {
    let mut digits = Vec::new();
    let mut spell_out_digits = create_digits();
    for ch in line.chars() {
        match char_to_digit(ch) {
            Some(num) => {
                digits.push(num);
            }
            None => (),
        }
        for dg in spell_out_digits.iter_mut() {
            match dg.update(ch.to_ascii_lowercase()) {
                Some(digit) => digits.push(digit),
                None => (),
            }
        }
    }
    if digits.len() == 0 {
        panic!("There must be number in each line! {}", line)
    }
    let first_digit = digits.first().unwrap();
    let last_digit = digits.last().unwrap();
    let result = (first_digit * 10 + last_digit) as u32;
    return result;
}
