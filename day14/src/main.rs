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
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop();
                }
                Ok(Some(line))
            }
            Err(e) => Err(e),
        }
    }
}

// function
fn read_file(mut line_reader: Line) -> Result<Puzzle, Error<'static>> {
    let mut puzzle = Vec::new();
    let mut row = Vec::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                for ch in line.chars() {
                    match ch {
                        '.' => row.push(Tile::Empty),
                        '#' => row.push(Tile::Square),
                        'O' => row.push(Tile::Round),
                        _ => unreachable!("There should only '.', 'O', or '#'"),
                    }
                }
                puzzle.push(std::mem::take(&mut row));
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
    Ok(puzzle)
}

// #################################################################################
// #################################################################################
// #################################################################################

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Round,
    Square,
    Empty,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Square => write!(f, "#"),
            Tile::Round => write!(f, "O"),
        }
    }
}

type Puzzle = Vec<Vec<Tile>>;
type Cache = HashMap<Vec<Tile>, Vec<Tile>>;

fn print_puzzle(puzzle: &Puzzle) {
    for row in puzzle {
        for tile in row {
            print!("{}", tile);
        }
        println!()
    }
}

fn tilt_north(puzzle: &mut Puzzle, x_len: usize, y_len: usize, cache: &mut Cache) {
    let mut stop = 0;
    for x in 0..x_len {
        let mut column = Vec::new();
        for y in 0..y_len {
            column.push(puzzle[y][x].clone());
        }
        if let Some(y_vals) = cache.get(&column) {
            for (y, y_val) in y_vals.iter().enumerate() {
                puzzle[y][x] = y_val.clone();
            }
            // println!("cache!");
        } else {
            let mut new_column = column.clone();
            for (y, tile) in column
                .iter()
                .enumerate()
                .filter(|&t| t.1 != &Tile::Empty)
                .collect::<Vec<_>>()
            {
                if tile == &Tile::Round {
                    if stop != y {
                        puzzle[y][x] = Tile::Empty;
                        puzzle[stop][x] = Tile::Round;
                        new_column[y] = Tile::Empty;
                        new_column[stop] = Tile::Round;
                    }
                    stop += 1;
                } else {
                    stop = y + 1;
                }
            }
            cache.insert(column, new_column);
            stop = 0;
        }
        // for y in 0..y_len {
        //     if puzzle[y][x] == Tile::Round {
        //         if stop != y {
        //             puzzle[y][x] = Tile::Empty;
        //             puzzle[stop][x] = Tile::Round;
        //         }
        //         stop += 1;
        //     } else if puzzle[y][x] == Tile::Square {
        //         stop = y + 1;
        //     }
        // }
    }
}

fn tilt_south(puzzle: &mut Puzzle, x_len: usize, y_len: usize, cache: &mut Cache) {
    let mut stop = y_len - 1;
    for x in 0..x_len {
        let mut column = Vec::new();
        for y in 0..y_len {
            column.push(puzzle[y][x].clone());
        }
        if let Some(y_vals) = cache.get(&column) {
            for (y, y_val) in y_vals.iter().enumerate() {
                puzzle[y][x] = y_val.clone();
            }
            // println!("cache!");
        } else {
            let mut new_column = column.clone();
            for (y, tile) in column
                .iter()
                .enumerate()
                .rev()
                .filter(|&t| t.1 != &Tile::Empty)
                .collect::<Vec<_>>()
            {
                if tile == &Tile::Round {
                    if stop != y {
                        puzzle[y][x] = Tile::Empty;
                        puzzle[stop][x] = Tile::Round;
                        new_column[y] = Tile::Empty;
                        new_column[stop] = Tile::Round;
                    }
                    stop = std::cmp::max(0, stop as i32 - 1 as i32) as usize;
                } else {
                    stop = std::cmp::max(0, y as i32 - 1 as i32) as usize;
                }
            }
            cache.insert(column, new_column);
            stop = y_len - 1;
        }
    }
}

fn tilt_west(puzzle: &mut Puzzle, y_len: usize, cache: &mut Cache) {
    let mut stop = 0;
    for y in 0..y_len {
        let row = puzzle[y].clone();
        if let Some(x_vals) = cache.get(&row) {
            puzzle[y] = x_vals.clone();
            // println!("cache!");
        } else {
            let mut new_row = row.clone();
            for (x, tile) in row
                .iter()
                .enumerate()
                .filter(|&t| t.1 != &Tile::Empty)
                .collect::<Vec<_>>()
            {
                if tile == &Tile::Round {
                    if stop != x {
                        // println!("uwu");
                        puzzle[y][x] = Tile::Empty;
                        puzzle[y][stop] = Tile::Round;
                        new_row[x] = Tile::Empty;
                        new_row[stop] = Tile::Round;
                    }
                    stop += 1;
                } else {
                    stop = x + 1;
                }
                // println!("{} {} {}", tile, x, stop);
            }
            cache.insert(row, new_row);
            stop = 0;
        }
    }
}

fn tilt_east(puzzle: &mut Puzzle, x_len: usize, y_len: usize, cache: &mut Cache) {
    let mut stop = x_len - 1;
    for y in 0..y_len {
        let row = puzzle[y].clone();
        if let Some(x_vals) = cache.get(&row) {
            puzzle[y] = x_vals.clone();
            // println!("cache!");
        } else {
            let mut new_row = row.clone();
            for (x, tile) in row
                .iter()
                .enumerate()
                .rev()
                .filter(|&t| t.1 != &Tile::Empty)
                .collect::<Vec<_>>()
            {
                if tile == &Tile::Round {
                    if stop != x {
                        puzzle[y][x] = Tile::Empty;
                        puzzle[y][stop] = Tile::Round;
                        new_row[x] = Tile::Empty;
                        new_row[stop] = Tile::Round;
                    }
                    stop = std::cmp::max(0, stop as i32 - 1 as i32) as usize;
                } else {
                    stop = std::cmp::max(0, x as i32 - 1 as i32) as usize;
                }
            }
            cache.insert(row, new_row);
            stop = x_len - 1;
        }
    }
}

fn part_one(mut puzzle: Puzzle) {
    let x_len = puzzle[0].len();
    let y_len = puzzle.len();
    tilt_north(&mut puzzle, x_len, y_len, &mut HashMap::new());
    print_puzzle(&puzzle);
    let mut total = 0;
    let y_len = puzzle.len();
    for (i, row) in puzzle.iter().enumerate() {
        let c = row.iter().filter(|&r| r == &Tile::Round).count();
        total += c * (y_len - i);
    }
    println!("Total: {}", total);
}

fn part_two(mut puzzle: Puzzle) {
    let x_len = puzzle[0].len();
    let y_len = puzzle.len();
    let mut cache_north = HashMap::new();
    let mut cache_west = HashMap::new();
    let mut cache_south = HashMap::new();
    let mut cache_east = HashMap::new();
    let mut puzzles: HashMap<Puzzle, Puzzle> = HashMap::new();
    // first time
    tilt_north(&mut puzzle, x_len, y_len, &mut cache_north);
    tilt_west(&mut puzzle, y_len, &mut cache_west);
    tilt_south(&mut puzzle, x_len, y_len, &mut cache_south);
    tilt_east(&mut puzzle, x_len, y_len, &mut cache_east);
    let mut puzzle_first = puzzle.clone();
    let mut starting = 1;
    // find up to the going back to loop
    loop {
        if let Some(_) = puzzles.get(&puzzle) {
            break;
        } else {
            tilt_north(&mut puzzle, x_len, y_len, &mut cache_north);
            tilt_west(&mut puzzle, y_len, &mut cache_west);
            tilt_south(&mut puzzle, x_len, y_len, &mut cache_south);
            tilt_east(&mut puzzle, x_len, y_len, &mut cache_east);
            puzzles.insert(std::mem::take(&mut puzzle_first), puzzle.clone());
            puzzle_first = puzzle.clone();
            starting += 1;
        }
    }
    println!("up to {} cycles", starting);
    let mut period = 0;
    let ori_puzzle = puzzle.clone();
    // find period
    loop {
        puzzle = puzzles.get(&puzzle).unwrap().clone();
        period += 1;
        if puzzle == ori_puzzle {
            break;
        }
    }
    println!("period: {}", period);
    let num_cycle = 1_000_000_000 - starting;
    let end = num_cycle % period;
    for _ in 0..end {
        puzzle = puzzles.get(&puzzle).unwrap().clone();
    }
    print_puzzle(&puzzle);
    let total = calc_score(&puzzle);
    println!("Total: {}", total);
}

fn calc_score(puzzle: &Puzzle) -> usize {
    let mut total = 0;
    let y_len = puzzle.len();
    for (i, row) in puzzle.iter().enumerate() {
        let c = row.iter().filter(|&r| r == &Tile::Round).count();
        total += c * (y_len - i);
    }
    total
}
