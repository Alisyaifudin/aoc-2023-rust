use std::collections::HashMap;
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
    // let file_path = root_dir.join("src").join("sample-3.txt");
    let file_path = root_dir.join("src").join("input.txt");

    // Create a Line instance from a file
    let line_reader = Line::new(file_path).map_err(Error::Io)?;

    match args.get(1) {
        None => return Err(Error::NoArg("Missing argument. Use '1' or '2'")),
        Some(arg) => {
            match arg.as_str() {
                "1" => part_one(line_reader)?,
                "2" => part_two(line_reader)?,
                "3" => part_extra(line_reader)?,
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

// function

fn read_file(
    mut line_reader: Line,
) -> Result<(Vec<Instruction>, HashMap<String, (String, String)>), Error<'static>> {
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

    let mut map = HashMap::new();
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
                map.insert(head.into(), (left.into(), right.into()));
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

    Ok((instructions, map))
}

// part one
fn part_one(line_reader: Line) -> Result<(), Error<'static>> {
    let (instructions, map) = read_file(line_reader)?;

    // println!("{:?}", instructions);

    let mut current = "AAA";
    let end = "ZZZ";
    let mut counter = 0;
    'outer: loop {
        for instruction in &instructions {
            // println!("{}: current: {}, go to {:?}", counter, current, instruction);
            if current == end {
                break 'outer;
            }
            match map.get(current) {
                None => panic!("No where to goooo!"),
                Some(dir) => {
                    // println!("map {:?}", dir);
                    match instruction {
                        Instruction::Left => current = &dir.0,
                        Instruction::Right => current = &dir.1,
                    }
                    counter += 1;
                }
            }
        }
    }
    println!("total: {}", counter);

    Ok(())
}

// part extra
fn part_extra(line_reader: Line) -> Result<(), Error<'static>> {
    let (instructions, map) = read_file(line_reader)?;

    // HJA      XDA     XSA     CFA     HPA     AAA
    let mut current = "AAA".to_string();
    let mut traces: Vec<(String, Instruction)> = Vec::new();
    // let mut traces: Vec<&str> = vec![current];
    // let mut mark: Option<(usize, &str, Instruction)> = None;
    // let end = "ZZZ";
    let mut counter = 0;
    'outer: loop {
        for instruction in &instructions {
            // println!("{}: current: {}, go to {:?}", counter, current, instruction);
            let trace = traces
                .iter()
                .enumerate()
                .find(|&(_, t)| t.0 == current && &t.1 == instruction);
            if let Some((i, tr)) = trace {
                println!("{} {:?}", tr.0, tr.1);
                println!("i={i} counter={counter} interval={}", counter - i);
                break 'outer;
            }
            traces.push((current.clone(), instruction.clone()));
            match map.get(&current) {
                None => panic!("No where to goooo!"),
                Some(dir) => {
                    // println!("{counter} {current} {instruction:?}");
                    print!("\r");
                    match instruction {
                        Instruction::Left => current = dir.0.clone(),
                        Instruction::Right => current = dir.1.clone(),
                    }
                    // traces.push(current);
                    counter += 1;
                    // if current.ends_with("Z") {
                    //     if let Some(m) = &mut mark {
                    //         if m.1 == current && &m.2 == instruction {
                    //             println!("finish: {counter} {current} {instruction:?}");
                    //             println!("i = {}", m.0);
                    //             println!("counter = {}", counter);
                    //             println!("interval = {}", counter - m.0);
                    //             break 'outer;
                    //         } else {
                    //             mark = Some((counter, current, instruction.clone()));
                    //             println!("find: {counter} {current} {instruction:?}");
                    //             println!();
                    //         }
                    //     } else {
                    //         mark = Some((counter, current, instruction.clone()));
                    //         println!("find: {counter} {current} {instruction:?}");
                    //         println!();
                    //     }
                    // }
                    // print!("{counter} {current}");
                    if counter == 10_000_000 {
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("\ntotal: {}", counter);

    Ok(())
}

// part two
fn part_two(line_reader: Line) -> Result<(), Error<'static>> {
    let (instructions, map) = read_file(line_reader)?;

    let mut starts = map
        .keys()
        .filter(|s| s.ends_with("A"))
        .cloned()
        .collect::<Vec<String>>();
    starts.sort();
    starts = starts
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, _)| i < &6 && i > &2)
        .map(|s| s.1)
        .collect::<Vec<_>>();

    let mut counter = 0;
    'outer: loop {
        for instruction in &instructions {
            let currents = starts.clone();
            starts.clear();

            let mut is_done = true;
            for current in &currents {
                if !current.ends_with("Z") {
                    is_done = false;
                    break;
                }
            }
            if is_done {
                println!("\n{}: current: {:?}, go to {:?}", counter, currents, instruction);
                break 'outer;
            }
            for current in &currents {
                // println!("{}: current: {}, go to {:?}", counter, current, instruction);
                match map.get(current) {
                    None => panic!("No where to goooo!"),
                    Some(dir) => {
                        // println!("map {:?}", dir);
                        match instruction {
                            Instruction::Left => starts.push(dir.0.clone()),
                            Instruction::Right => starts.push(dir.1.clone()),
                        }
                    }
                }
            }
            counter += 1;
            // println!();
            if counter % 1000 == 0 {
                print!("\r{}",counter);
            }
            // print!("{}", counter);
            // if counter == 1000 {
            //     break 'outer;
            // }
        }
    }

    println!("total: {}", counter);

    Ok(())
}

fn part_ex(line_reader: Line) -> Result<(), Error<'static>> {
    let (instructions, map) = read_file(line_reader)?;

    let mut starts = map
        .keys()
        .filter(|s| s.ends_with("A"))
        .cloned()
        .collect::<Vec<String>>();
    starts.sort();
    starts = starts
        .iter()
        .cloned()
        .enumerate()
        .filter(|(i, _)| i < &6 && i > &2)
        .map(|s| s.1)
        .collect::<Vec<_>>();

    let mut counter = 0;
    'outer: loop {
        for instruction in &instructions {
            let currents = starts.clone();
            starts.clear();

            let mut is_done = true;
            for current in &currents {
                if !current.ends_with("Z") {
                    is_done = false;
                    break;
                }
            }
            if is_done {
                println!("\n{}: current: {:?}, go to {:?}", counter, currents, instruction);
                break 'outer;
            }
            for current in &currents {
                // println!("{}: current: {}, go to {:?}", counter, current, instruction);
                match map.get(current) {
                    None => panic!("No where to goooo!"),
                    Some(dir) => {
                        // println!("map {:?}", dir);
                        match instruction {
                            Instruction::Left => starts.push(dir.0.clone()),
                            Instruction::Right => starts.push(dir.1.clone()),
                        }
                    }
                }
            }
            counter += 1;
            // println!();
            if counter % 1000 == 0 {
                print!("\r{}",counter);
            }
            // print!("{}", counter);
            // if counter == 1000 {
            //     break 'outer;
            // }
        }
    }

    println!("total: {}", counter);

    Ok(())
}