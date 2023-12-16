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

// #################################################################################
// main
// part one

fn read_file_part_one(mut line_reader: Line) -> Result<Vec<(usize, usize)>, Error<'static>> {
    // read first line
    let first_line = line_reader
        .read()
        .map_err(Error::Io)?
        .expect("No first line found");
    let splitted = first_line.split(":").collect::<Vec<&str>>();
    let times = (*splitted.get(1).expect("No times?"))
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().expect("Cannot parse the number"))
        .collect::<Vec<usize>>();

    let second_line = line_reader
        .read()
        .map_err(Error::Io)?
        .expect("No second line found");
    let splitted = second_line.split(":").collect::<Vec<&str>>();
    let distances = (*splitted.get(1).expect("No distances?"))
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().expect("Cannot parse the number"))
        .collect::<Vec<usize>>();
    let res = times
        .iter()
        .zip(distances.iter())
        .map(|(&a, &b)| (a, b))
        .collect::<Vec<_>>();
    Ok(res)
}

fn part_one(line_reader: Line) -> Result<(), Error<'static>> {
    let records = read_file_part_one(line_reader)?;
    let mut result = 1;
    for (time, distance) in records {
        for t1 in 1..time {
            let d = t1 * (time - t1);
            if d > distance {
                let t2 = time - t1;
                println!("{} {}", t1, t2);
                let num = t2 - t1 + 1;
                result *= num;
                break;
            }
        }
    }
    println!("result = {}", result);
    Ok(())
}

// part two

fn read_file_part_two(mut line_reader: Line) -> Result<(usize, usize), Error<'static>> {
    // read first line
    let first_line = line_reader
        .read()
        .map_err(Error::Io)?
        .expect("No first line found");
    let splitted = first_line.split(":").collect::<Vec<&str>>();
    let time = (*splitted.get(1).expect("No times?"))
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .expect("Cannot parse the number");

    let second_line = line_reader
        .read()
        .map_err(Error::Io)?
        .expect("No second line found");
    let splitted = second_line.split(":").collect::<Vec<&str>>();
    let distance = (*splitted.get(1).expect("No distances?"))
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .expect("Cannot parse the number");
    Ok((time, distance))
}

fn part_two(line_reader: Line) -> Result<(), Error<'static>> {
    let (time, distance) = read_file_part_two(line_reader)?;
    println!("time: {}", time);
    println!("distance: {}", distance);
    for t1 in 1..time {
        let d = t1 * (time - t1);
        if d > distance {
            let t2 = time - t1;
            println!("t1={}, t2={}", t1, t2);
            let num = t2 - t1 + 1;
            println!("result = {}", num);
            break;
        }
    }
    Ok(())
}
