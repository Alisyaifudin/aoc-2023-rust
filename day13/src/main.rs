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
    let puzzles = read_file(line_reader)?;

    match args.get(1) {
        None => return Err(Error::NoArg("Missing argument. Use '1' or '2'")),
        Some(arg) => {
            match arg.as_str() {
                "1" => part_one(puzzles),
                "2" => part_two(puzzles),
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

// function
fn read_file(mut line_reader: Line) -> Result<Vec<Puzzle>, Error<'static>> {
    let mut puzzles = Vec::new();
    let mut puzzle = Vec::new();
    let mut row = Vec::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                if line == "\n" {
                    puzzles.push(std::mem::take(&mut puzzle));
                    continue;
                }
                for ch in line.chars() {
                    match ch {
                        '.' => row.push(Tile::Ash),
                        '#' => row.push(Tile::Rock),
                        '\n' => (),
                        _ => unreachable!("There should only '.' or '#'"),
                    }
                }
                puzzle.push(std::mem::take(&mut row));
            }
            Ok(None) => {
                // End of file reached
                puzzles.push(std::mem::take(&mut puzzle));
                break;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(Error::Io(e));
            }
        }
    }
    Ok(puzzles)
}

// #################################################################################
// #################################################################################
// #################################################################################

// enum
#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Rock,
    Ash,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

type Puzzle = Vec<Vec<Tile>>;

#[allow(dead_code)]
fn print_puzzle(puzzle: &Puzzle) {
    for row in puzzle {
        for t in row {
            print!("{}", t);
        }
        println!()
    }
}

#[derive(PartialEq, Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

fn solve(puzzle: &Puzzle, reflection: Option<&Reflection>) -> Option<(usize, Reflection)> {
    // search vertical reflection
    let mut column_index = None;
    let x_len = puzzle[0].len();
    let y_len = puzzle.len();
    // print_puzzle(puzzle);
    let mut valid = true;

    for b in 1..x_len {
        if let Some(r) = reflection {
            match r {
                Reflection::Vertical(s) => {
                    if &b == s {
                        continue;
                    }
                }
                _ => (),
            }
        }
        let x_min = std::cmp::max(0, (2 * b) as i32 - (x_len as i32)) as usize;
        let x_max = std::cmp::min(x_len, b * 2);
        // println!("max {}, min {}, b {}", x_max, x_min, b);
        for y in 0..y_len {
            for dx in 0..b - x_min {
                // if b == 5 {
                //     println!("({},{}) = ({},{})", x_min + dx, y, x_max - dx, y);
                //     println!("({}) = ({})", puzzle[y][x_min+dx], puzzle[y][x_max - dx -1]);
                // }
                if puzzle[y][x_min + dx] != puzzle[y][x_max - dx - 1] {
                    valid = false;

                    break;
                }
            }
        }
        if valid {
            column_index = Some(b);
            break;
        }
        valid = true;
    }
    if let Some(i) = column_index {
        return Some((i, Reflection::Vertical(i)));
    }
    let mut row_index = None;
    let mut valid = true;
    for b in 1..y_len {
        if let Some(r) = reflection {
            match r {
                Reflection::Horizontal(s) => {
                    if &b == s {
                        continue;
                    }
                }
                _ => (),
            }
        }
        let y_min = std::cmp::max(0, (2 * b) as i32 - (y_len as i32)) as usize;
        let y_max = std::cmp::min(y_len, b * 2);
        for x in 0..x_len {
            for dy in 0..b - y_min {
                if puzzle[y_min + dy][x] != puzzle[y_max - dy - 1][x] {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            row_index = Some(b);
            break;
        }
        valid = true;
    }
    if let Some(i) = row_index {
        return Some((i * 100, Reflection::Horizontal(i)));
    }
    None
}

fn part_one(puzzles: Vec<Puzzle>) -> usize {
    // search vertical reflection
    let mut total = 0;
    for (i, puzzle) in puzzles.iter().enumerate() {
        let (subtotal, reflection) =
            solve(puzzle, None).expect("Impossible, every puzzle must have a mirror 🤯");
        println!("{}: {} {:?}", i + 1, subtotal, reflection);
        print_puzzle(&puzzle);
        total += subtotal;
    }
    println!("Total: {}", total);
    total
}

fn part_two(puzzles: Vec<Puzzle>) -> usize {
    // search vertical reflection
    let mut total = 0;
    for (i, puzzle) in puzzles.iter().enumerate() {
        let (old, reflection) =
            solve(puzzle, None).expect("Impossible, every puzzle must have a mirror 🤯");

        let mut new_puzzle = puzzle.clone();
        let x_len = puzzle[0].len();
        let y_len = puzzle.len();
        'outer: for x in 0..x_len {
            for y in 0..y_len {
                new_puzzle[y][x] = match new_puzzle[y][x] {
                    Tile::Ash => Tile::Rock,
                    Tile::Rock => Tile::Ash,
                };
                let subtotal = solve(&new_puzzle, Some(&reflection));
                if let Some(s) = subtotal {
                    println!("{}: new = {} {:?}", i + 1, s.0, s.1);
                    println!("{}: old = {} {:?}", i + 1, old, reflection);
                    print_puzzle(&new_puzzle);
                    total += s.0;
                    break 'outer;
                }
                new_puzzle[y][x] = match new_puzzle[y][x] {
                    Tile::Ash => Tile::Rock,
                    Tile::Rock => Tile::Ash,
                };
            }
        }
    }
    println!("Total: {}", total);
    total
}
