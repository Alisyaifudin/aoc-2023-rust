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

    let line_reader = Line::new(file_path).map_err(Error::Io)?;

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

    let map = read_file(line_reader)?;

    func(map);

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
// enum
#[derive(Clone, Debug, PartialEq)]
enum Object {
    Galaxy,
    Empty,
}

impl Object {
    fn new(ch: char) -> Self {
        match ch {
            '.' => Object::Empty,
            '#' => Object::Galaxy,
            _ => unreachable!("Unknown char, should be only `.` or `#`"),
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Empty => write!(f, "."),
            Object::Galaxy => write!(f, "#"),
        }
    }
}

#[derive(Clone, Debug)]
struct Point {
    object: Object,
    expand_x: bool,
    expand_y: bool,
}

impl Point {
    fn new(object: Object) -> Self {
        Point {
            object,
            expand_x: false,
            expand_y: false,
        }
    }
    fn expand_x(&mut self) {
        self.expand_x = true;
    }
    fn expand_y(&mut self) {
        self.expand_y = true;
    }
}

#[derive(Clone, Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Galaxy { x, y }
    }
}

type Points = Vec<Vec<Point>>;

// function
fn read_file(mut line_reader: Line) -> Result<Points, Error<'static>> {
    let mut points = Vec::new();
    let mut row = Vec::new();
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                // println!("{}", line);
                for ch in line.chars() {
                    let obj = Object::new(ch);
                    row.push(Point::new(obj));
                }
                points.push(std::mem::take(&mut row));
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
    // println!();
    Ok(points)
}

fn distance(
    points: &Points,
    factor: usize,
    first_galaxy: &Galaxy,
    second_galaxy: &Galaxy,
) -> usize {
    let x_steps = first_galaxy.x.abs_diff(second_galaxy.x);
    let x_0 = std::cmp::min(first_galaxy.x, second_galaxy.x);
    let y_steps = first_galaxy.y.abs_diff(second_galaxy.y);
    let y_0 = std::cmp::min(first_galaxy.y, second_galaxy.y);
    let mut counter: usize = 0; //
                                // case 1: x_steps = 0
    if x_steps == 0 {
        for y in y_0 + 1..=y_0 + y_steps {
            if points[y][x_0].expand_y {
                counter += factor;
            } else {
                counter += 1
            }
        }
        return counter;
    }
    // case 2: y_steps = 0
    if y_steps == 0 {
        for x in x_0 + 1..=x_0 + x_steps {
            if points[y_0][x].expand_x {
                counter += factor;
            } else {
                counter += 1
            }
        }
        return counter;
    }
    // case 3: both non-zero
    for x in x_0 + 1..=x_0 + x_steps {
        if points[y_0][x].expand_x {
            counter += factor;
        } else {
            counter += 1
        }
    }
    for y in y_0 + 1..=y_0 + y_steps {
        if points[y][x_0].expand_y {
            counter += factor;
        } else {
            counter += 1
        }
    }
    return counter;
}

fn calculate(mut points: Points, factor: usize) {
    // expand row
    let y_dim = points.len();
    let x_dim = points[0].len();
    for y in 0..y_dim {
        let row = &mut points[y];
        if row.iter().all(|c| c.object == Object::Empty) {
            for x in 0..x_dim {
                points[y][x].expand_y();
            }
        }
    }
    // expand column
    for x in 0..x_dim {
        for y in 0..y_dim {
            if points[y][x].object == Object::Galaxy {
                break;
            }
            if y == y_dim - 1 {
                for i in 0..y_dim {
                    points[i][x].expand_x();
                }
            }
        }
    }

    let mut galaxies: Vec<Galaxy> = Vec::new();

    for (y, row) in points.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if point.object == Object::Galaxy {
                galaxies.push(Galaxy::new(x, y))
            }
        }
    }
    let mut distances = Vec::new();
    for i in 0..galaxies.len() - 1 {
        let first_galaxy = &galaxies[i];
        for j in i + 1..galaxies.len() {
            let second_galaxy = &galaxies[j];
            let distance = distance(&points, factor, &first_galaxy, &second_galaxy);
            distances.push(distance);
        }
    }
    let total = distances.iter().sum::<usize>();
    println!("{}", total);
}

// main 

fn part_one(points: Points) {
    calculate(points, 2);
}

fn part_two(points: Points) {
    calculate(points, 1_000_000);
}