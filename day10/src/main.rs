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
    // let file_path = root_dir.join("src").join("sample-7.txt");
    let file_path = root_dir.join("src").join("input.txt");
    let animal_pipe = Pipe {
        first: Face::North,
        second: Face::West,
        heading: Some(Face::North), 
        // choosing animal heading is important
        // it's either first or second, choose the one such that the outer loop
        // is in counter-clockwise direction
        // just try it out
    };
    // for sample
    
    // let animal_heading = Face::North; // for real data

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

    let map = read_file(line_reader, animal_pipe)?;

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

#[derive(Debug, PartialEq, Clone)]
enum Face {
    West,
    East,
    North,
    South,
}

#[derive(PartialEq, Clone, Debug)]
struct Pipe {
    first: Face,
    second: Face,
    heading: Option<Face>,
}

impl Pipe {
    fn new(first: Face, second: Face) -> Self {
        Pipe {
            first,
            second,
            heading: None,
        }
    }
    fn moves(&mut self, from: Face) -> Face {
        match from {
            face if opposite(&face) == self.first => {
                self.heading = Some(self.second.clone());
            }
            face if opposite(&face) == self.second => {
                self.heading = Some(self.first.clone());
            }
            _ => unreachable!("impossible direction"),
        }
        self.heading.clone().unwrap()
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.first, &self.second) {
            (Face::North, Face::South) => write!(f, "|"),
            (Face::West, Face::East) => write!(f, "-"),
            (Face::North, Face::East) => write!(f, "L"),
            (Face::North, Face::West) => write!(f, "J"),
            (Face::South, Face::West) => write!(f, "7"),
            (Face::South, Face::East) => write!(f, "F"),
            _ => write!(f, "😵‍💫"),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Loc {
    Inside,
    Outside,
}

#[derive(PartialEq, Clone)]
enum Tile {
    Nil(Loc),
    Animal(Pipe),
    Pipe(Pipe),
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Tile::Nil(_) => write!(f, "."),
            Tile::Animal(_) => write!(f, "S"),
            Tile::Pipe(pipe) => write!(f, "{}", pipe),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Tile::Nil(Loc::Inside) => write!(f, "I"),
            Tile::Nil(Loc::Outside) => write!(f, "O"),
            Tile::Animal(_) => write!(f, "S"),
            Tile::Pipe(pipe) => write!(f, "{}", pipe),
        }
    }
}

fn read_tile(ch: char, animal_pipe: Pipe) -> Tile {
    match ch {
        '.' => Tile::Nil(Loc::Outside), // by default is outside
        'S' => Tile::Animal(animal_pipe),
        '|' => Tile::Pipe(Pipe::new(Face::North, Face::South)),
        '-' => Tile::Pipe(Pipe::new(Face::West, Face::East)),
        'L' => Tile::Pipe(Pipe::new(Face::North, Face::East)),
        'J' => Tile::Pipe(Pipe::new(Face::North, Face::West)),
        '7' => Tile::Pipe(Pipe::new(Face::South, Face::West)),
        'F' => Tile::Pipe(Pipe::new(Face::South, Face::East)),
        _ => unreachable!(),
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    x_dim: usize,
    y_dim: usize,
    start: (usize, usize),
}

impl Map {
    fn find(&mut self, x: usize, y: usize) -> Tile {
        self.tiles[y][x].clone()
    }
    fn update(&mut self, x: usize, y: usize, updated: &Tile) {
        if let Some(tiles) = self.tiles.get_mut(y) {
            if let Some(tile) = tiles.get_mut(x) {
                *tile = updated.clone();
            }
        }
    }
}

fn read_file(mut line_reader: Line, animal_pipe: Pipe) -> Result<Map, Error<'static>> {
    let mut tiles = Vec::new();
    let mut current_tiles = Vec::new();
    let mut start = (0, 0);
    let mut y: usize = 0;
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                for (x, ch) in line.chars().enumerate() {
                    let tile = read_tile(ch, animal_pipe.clone());
                    if tile == Tile::Animal(animal_pipe.clone()) {
                        start = (x, y);
                    }
                    current_tiles.push(tile);
                }
                y += 1;
                tiles.push(std::mem::take(&mut current_tiles));
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
    let x_dim = tiles[0].len();
    let y_dim = tiles.len();
    Ok(Map {
        tiles,
        x_dim,
        y_dim,
        start,
    })
}

fn opposite(direction: &Face) -> Face {
    match direction {
        Face::East => Face::West,
        Face::North => Face::South,
        Face::West => Face::East,
        Face::South => Face::North,
    }
}

fn next_move(moves: &Face, current: (usize, usize), x_max: usize, y_max: usize) -> (usize, usize) {
    let mut x_pos = current.0;
    let mut y_pos = current.1;
    match moves {
        Face::East => x_pos += 1,
        Face::West => x_pos -= 1,
        Face::North => y_pos -= 1,
        Face::South => y_pos += 1,
    }
    if x_pos > x_max {
        panic!("Out of x bound T_T");
    }
    if y_pos > y_max {
        panic!("Out of y bound T_T");
    }
    (x_pos, y_pos)
}

fn part_one(mut map: Map) {
    let mut current = map.start.clone();
    let x_max = map.x_dim;
    let y_max = map.y_dim;
    let mut moves = Face::North; // arbitrary number, will change immediately
    let mut traces = Vec::new();
    let mut counter: usize = 0;
    loop {
        let mut tile = map.find(current.0, current.1);
        match &mut tile {
            Tile::Animal(pipe) => {
                if traces.len() > 1
                    && traces
                        .iter()
                        .find(|&t| t == &Tile::Animal(pipe.clone()))
                        .is_some()
                {
                    counter += 1;
                    break;
                }
                moves = pipe.heading.clone().unwrap();
            }
            Tile::Nil(_) => panic!("Cannot step on dot"),
            Tile::Pipe(ref mut pipe) => moves = pipe.moves(moves),
        }
        if traces.len() == 0 {
            print!("[{:?} {:?}]", tile, moves);
        } else {
            print!(" -> [ {:?} {:?}]", tile, moves);
        }
        traces.push(tile);
        let next_moves = next_move(&moves, current, x_max, y_max);
        counter += 1;
        current = next_moves;
    }
    println!("\n{}", counter / 2);
}

fn part_two(mut map: Map) {
    let mut current = map.start.clone();
    let x_max = map.x_dim;
    let y_max = map.y_dim;

    let mut moves = Face::North; // arbitrary number, will change immediately
    let mut traces = Vec::new();
    loop {
        let mut tile = map.find(current.0, current.1);
        match &mut tile {
            Tile::Animal(pipe) => {
                if traces.len() > 1
                    && traces
                        .iter()
                        .find(|&t| t == &Tile::Animal(pipe.clone()))
                        .is_some()
                {
                    break;
                }
                moves = pipe.heading.clone().unwrap();
            }
            Tile::Nil(_) => panic!("Cannot step on dot"),
            Tile::Pipe(ref mut pipe) => moves = pipe.moves(moves),
        }
        map.update(current.0, current.1, &tile);
        traces.push(tile);
        let next_moves = next_move(&moves, current, x_max, y_max);
        current = next_moves;
    }
    let mut walker = Loc::Outside;
    let mut counter = 0;
    for seq_of_tiles in map.tiles.iter_mut() {
        for tile in seq_of_tiles.iter_mut() {
            // match tile {
            //     Tile::Animal(heading) => println!("Animal: {:?}", heading),
            //     Tile::Pipe(pipe) => println!("{:?}", pipe),
            //     _ => (),
            // }
            match tile {
                Tile::Animal(ref pipe) => walker = switch_loc(walker, pipe.clone()),
                Tile::Pipe(ref pipe) => {
                    if pipe.heading.is_none() {
                        match walker {
                            Loc::Inside => {
                                *tile = Tile::Nil(Loc::Inside);
                                counter += 1;
                            }
                            Loc::Outside => *tile = Tile::Nil(Loc::Outside),
                        }
                    } else {
                        walker = switch_loc(walker, pipe.clone())
                    }
                }
                Tile::Nil(_) => match walker {
                    Loc::Inside => {
                        *tile = Tile::Nil(Loc::Inside);
                        counter += 1;
                    }
                    Loc::Outside => *tile = Tile::Nil(Loc::Outside),
                },
            }
            print!("{}", tile);
        }
        walker = Loc::Outside;
        print!("\n");
    }
    println!("{}", counter);
}

fn switch_loc(current: Loc, pipe: Pipe) -> Loc {
    if let Some(heading) = pipe.heading {
        match heading {
            Face::East => current,
            Face::North => Loc::Outside,
            Face::South => Loc::Inside,
            Face::West => match pipe.first {
                Face::West => current,
                Face::North => Loc::Inside,
                Face::South => Loc::Outside,
                _ => unreachable!("Impossible"),
            },
        }
    } else {
        current
    }
}
