use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug)]
enum Invalid<'a> {
    Arg(&'a str),
    Line(&'a str),
    Cube(&'a str),
    Color(&'a str),
    Parse(ParseIntError),
    Id(&'a str),
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
    loop {
        match line_reader.read() {
            Ok(Some(line)) => total += func(line).map_err(Error::Inv)?,
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

#[derive(Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

const RED: u16 = 12;
const GREEN: u16 = 13;
const BLUE: u16 = 14;

impl Color {
    fn is_invalid(&self, num: u16) -> bool {
        match self {
            Color::Red => num > RED,
            Color::Green => num > GREEN,
            Color::Blue => num > BLUE,
        }
    }
}

fn create_map() -> HashMap<&'static str, Color> {
    let color_map: HashMap<&str, Color> = [
        ("red", Color::Red),
        ("green", Color::Green),
        ("blue", Color::Blue),
    ]
    .iter()
    .cloned()
    .collect();

    color_map
}

#[allow(dead_code)]
fn part_one<'a>(line: String) -> Result<u32, Invalid<'a>> {
    let splitted: Vec<&str> = line.split(":").collect();
    let game_id = splitted
        .get(0)
        .map(|&s| {
            s.split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .get(1)
                .map(|&id| id.parse::<u16>().map_err(Invalid::Parse))
                .ok_or(Invalid::Id("No Id"))
        })
        .ok_or(Invalid::Id("No Game <Id>"))???;

    let game = *splitted
        .get(1)
        .ok_or(Invalid::Line("The line does not contain ':'"))?;
    let color_map = create_map();
    let sets: Vec<&str> = game.split(";").collect();
    let mut valid = true;

    'outer: for set in sets {
        let cubes: Vec<&str> = set.split(",").collect();
        for cube in cubes {
            let cube_info: Vec<&str> = cube.split_ascii_whitespace().collect();
            let num = cube_info
                .get(0)
                .map(|&n| n.parse::<u16>().map_err(Invalid::Parse))
                .ok_or(Invalid::Cube("No number found"))??;
            let color = cube_info
                .get(1)
                .map(|&c| color_map.get(c).ok_or(Invalid::Color("Invalid color")))
                .ok_or(Invalid::Cube("No color found"))??;
            valid = !color.is_invalid(num);
            if !valid {
                break 'outer;
            }
        }
    }
    if valid {
        Ok(game_id as u32)
    } else {
        Ok(0)
    }
}

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn new() -> Self {
        Bag {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn update(&mut self, color: &Color, num: u32) {
        match color {
            Color::Blue => {
                if self.blue < num {
                    self.blue = num;
                }
            }
            Color::Green => {
                if self.green < num {
                    self.green = num;
                }
            }
            Color::Red => {
                if self.red < num {
                    self.red = num;
                }
            }
        }
    }
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[allow(dead_code)]
fn part_two<'a>(line: String) -> Result<u32, Invalid<'a>> {
    let splitted: Vec<&str> = line.split(":").collect();

    let game = *splitted
        .get(1)
        .ok_or(Invalid::Line("The line does not contain ':'"))?;
    let color_map = create_map();
    let sets: Vec<&str> = game.split(";").collect();
    let mut bag = Bag::new();
    for set in sets {
        let cubes: Vec<&str> = set.split(",").collect();
        for cube in cubes {
            let cube_info: Vec<&str> = cube.split_ascii_whitespace().collect();
            let num = cube_info
                .get(0)
                .map(|&n| n.parse::<u16>().map_err(Invalid::Parse))
                .ok_or(Invalid::Cube("No number found"))??;
            let color = cube_info
                .get(1)
                .map(|&c| color_map.get(c).ok_or(Invalid::Color("Invalid color")))
                .ok_or(Invalid::Cube("No color found"))??;
            bag.update(color, num as u32);
        }
    }
    // println!("{:?} = {}", bag, bag.power());
    Ok(bag.power())
}
