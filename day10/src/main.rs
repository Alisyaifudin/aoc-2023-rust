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
    let animal_kind = Kind::NorthWest; // change based on the input
    let moves = Direction::West; // change based on the input

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

    let map = read_file(line_reader, animal_kind)?;

    func(map, moves);

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
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, Clone)]
enum Kind {
    NorthSouth, // |
    WestEast,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
}

#[derive(PartialEq, Clone, Debug)]
struct Pipe {
    in_loop: bool,
    animal: bool,
    kind: Kind,
    dir: (Direction, Direction),
}

fn kind_to_dir(kind: &Kind) -> (Direction, Direction) {
    match kind {
        Kind::NorthEast => (Direction::North, Direction::East),
        Kind::NorthSouth => (Direction::North, Direction::South),
        Kind::NorthWest => (Direction::North, Direction::West),
        Kind::SouthEast => (Direction::South, Direction::East),
        Kind::SouthWest => (Direction::South, Direction::West),
        Kind::WestEast => (Direction::West, Direction::East),
    }
}

fn opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::East => Direction::West,
        Direction::North => Direction::South,
        Direction::West => Direction::East,
        Direction::South => Direction::North,
    }
}

impl Pipe {
    fn new(kind: Kind, animal: bool) -> Self {
        Pipe {
            in_loop: animal,
            animal,
            dir: kind_to_dir(&kind),
            kind,
        }
    }
    fn moves(&mut self, from: Direction) -> Direction {
        match from {
            dir if opposite(&dir) == self.dir.0 => {
                self.in_loop = true;
                return self.dir.1.clone();
            }
            dir if opposite(&dir) == self.dir.1 => {
                self.in_loop = true;
                return self.dir.0.clone();
            }
            _ => unreachable!("impossible direction"),
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            Kind::NorthSouth => write!(f, "|"),
            Kind::WestEast => write!(f, "-"),
            Kind::NorthEast => write!(f, "L"),
            Kind::NorthWest => write!(f, "J"),
            Kind::SouthWest => write!(f, "7"),
            Kind::SouthEast => write!(f, "F"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Loc {
    Inside,
    Outside,
}

impl Loc {
    fn opposite(loc: &Loc) -> Loc {
        match loc {
            Loc::Inside => Loc::Outside,
            Loc::Outside => Loc::Inside,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Nil(Loc),
    Pipe(Pipe),
}

// impl std::fmt::Debug for Tile {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Tile::Nil(_) => write!(f, "."),
//             Tile::Pipe(pipe) => {
//                 if pipe.animal {
//                     write!(f, "S")
//                 } else {
//                     write!(f, "{}", pipe)
//                 }
//             }
//         }
//     }
// }

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Tile::Nil(Loc::Inside) => write!(f, "I"),
            Tile::Nil(Loc::Outside) => write!(f, "O"),
            Tile::Pipe(pipe) => {
                if pipe.animal {
                    write!(f, "S")
                } else {
                    write!(f, "{}", pipe)
                }
            }
        }
    }
}

fn read_tile(ch: char, animal_kind: Kind) -> Tile {
    match ch {
        '.' => Tile::Nil(Loc::Outside), // by default is outside
        'S' => Tile::Pipe(Pipe::new(animal_kind, true)),
        '|' => Tile::Pipe(Pipe::new(Kind::NorthSouth, false)),
        '-' => Tile::Pipe(Pipe::new(Kind::WestEast, false)),
        'L' => Tile::Pipe(Pipe::new(Kind::NorthEast, false)),
        'J' => Tile::Pipe(Pipe::new(Kind::NorthWest, false)),
        '7' => Tile::Pipe(Pipe::new(Kind::SouthWest, false)),
        'F' => Tile::Pipe(Pipe::new(Kind::SouthEast, false)),
        _ => unreachable!("Invalid character"),
    }
}

#[derive(Debug)]
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
    fn update(&mut self, x: usize, y: usize, tile: &Tile) {
        self.tiles[y][x] = tile.clone();
    }
}

fn read_file(mut line_reader: Line, animal_kind: Kind) -> Result<Map, Error<'static>> {
    let mut tiles = Vec::new();
    let mut current_tiles = Vec::new();
    let mut start = (0, 0);
    let mut y: usize = 0;
    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                for (x, ch) in line.chars().enumerate() {
                    let tile = read_tile(ch, animal_kind.clone());
                    if let Tile::Pipe(pipe) = &tile {
                        if pipe.animal {
                            start = (x, y);
                        }
                    }
                    if tile == Tile::Nil(Loc::Outside) {
                        print!(".");
                    } else {
                        print!("{}", tile);
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
        println!();
    }
    println!();
    let x_dim = tiles[0].len();
    let y_dim = tiles.len();
    Ok(Map {
        tiles,
        x_dim,
        y_dim,
        start,
    })
}

fn next_move(
    moves: &Direction,
    current: (usize, usize),
    x_max: usize,
    y_max: usize,
) -> (usize, usize) {
    let mut x_pos: usize = current.0;
    let mut y_pos = current.1;
    match moves {
        Direction::East => x_pos += 1,
        Direction::West => x_pos -= 1,
        Direction::North => y_pos -= 1,
        Direction::South => y_pos += 1,
    }
    if x_pos > x_max {
        panic!("Out of x bound T_T");
    }
    if y_pos > y_max {
        panic!("Out of y bound T_T");
    }
    (x_pos, y_pos)
}

fn part_one(mut map: Map, mut moves: Direction) {
    let mut current = map.start.clone();
    let x_max = map.x_dim;
    let y_max = map.y_dim;
    let mut traces = Vec::new();
    let mut counter: usize = 0;
    loop {
        let mut tile = map.find(current.0, current.1);
        moves = match &mut tile {
            Tile::Nil(_) => panic!("Cannot step on dot"),
            Tile::Pipe(ref mut pipe) => {
                if pipe.animal {
                    if traces.len() != 0 {
                        break;
                    }
                    moves
                } else {
                    pipe.moves(moves)
                }
            }
        };
        traces.push(tile);
        let next_moves = next_move(&moves, current, x_max, y_max);
        counter += 1;
        current = next_moves;
    }
    println!("\n{}", counter / 2);
}

fn part_two(mut map: Map, mut moves: Direction) {
    let mut current = map.start.clone();
    let x_max = map.x_dim;
    let y_max = map.y_dim;
    // let mut traces = Vec::new();
    let mut running = false;
    loop {
        let mut tile = map.find(current.0, current.1);
        moves = match &mut tile {
            Tile::Nil(_) => panic!("Cannot step on dot"),
            Tile::Pipe(ref mut pipe) => {
                if pipe.animal {
                    if running {
                        break;
                    }
                    running = true;
                    moves
                } else {
                    let u = pipe.moves(moves);
                    u
                }
            }
        };
        map.update(current.0, current.1, &tile);
        let next_moves = next_move(&moves, current, x_max, y_max);
        current = next_moves;
    }
    let mut walker = Loc::Outside;
    let mut turn: Option<Kind> = None;
    let mut counter = 0;
    for tiles in map.tiles.iter_mut() {
        for tile in tiles {
            match tile {
                Tile::Nil(loc) => {
                    if walker == Loc::Inside {
                        counter += 1;
                    }
                    *loc = walker.clone();
                }
                Tile::Pipe(pipe) => {
                    if pipe.in_loop {
                        match pipe.kind {
                            Kind::NorthSouth => walker = Loc::opposite(&walker),
                            Kind::WestEast => (),
                            Kind::NorthEast => {
                                turn = Some(Kind::NorthEast);
                            }
                            Kind::SouthEast => {
                                turn = Some(Kind::SouthEast);
                            }
                            Kind::NorthWest => {
                                if let Some(t) = &turn {
                                    if t == &Kind::SouthEast {
                                        walker = Loc::opposite(&walker);
                                    }
                                    turn = None;
                                } else {
                                    unreachable!(
                                        "Cannot turn from west without turn to east first 😡"
                                    );
                                }
                            }
                            Kind::SouthWest => {
                                if let Some(t) = &turn {
                                    if t == &Kind::NorthEast {
                                        walker = Loc::opposite(&walker);
                                    }
                                    turn = None;
                                } else {
                                    unreachable!(
                                        "Cannot turn from west without turn to east first 😡"
                                    );
                                }
                            }
                        }
                    } else {
                        if walker == Loc::Inside {
                            counter += 1;
                        }
                        *tile = Tile::Nil(walker.clone());
                    }
                }
            }
            print!("{}", tile);
        }
        walker = Loc::Outside;
        print!("\n");
    }
    println!("\n{}", counter);

    // let mut current = map.start.clone();
    // let x_max = map.x_dim;
    // let y_max = map.y_dim;
    // let mut moves = Face::North; // arbitrary number, will change immediately
    // let mut traces = Vec::new();
    // loop {
    //     let mut tile = map.find(current.0, current.1);
    //     match &mut tile {
    //         Tile::Animal(pipe) => {
    //             if traces.len() > 1
    //                 && traces
    //                     .iter()
    //                     .find(|&t| t == &Tile::Animal(pipe.clone()))
    //                     .is_some()
    //             {
    //                 break;
    //             }
    //             moves = pipe.heading.clone().unwrap();
    //         }
    //         Tile::Nil(_) => panic!("Cannot step on dot"),
    //         Tile::Pipe(ref mut pipe) => moves = pipe.moves(moves),
    //     }
    //     map.update(current.0, current.1, &tile);
    //     traces.push(tile);
    //     let next_moves = next_move(&moves, current, x_max, y_max);
    //     current = next_moves;
    // }
    // let mut walker = Loc::Outside;
    // let mut counter = 0;
    // for seq_of_tiles in map.tiles.iter_mut() {
    //     for tile in seq_of_tiles.iter_mut() {
    //         // match tile {
    //         //     Tile::Animal(heading) => println!("Animal: {:?}", heading),
    //         //     Tile::Pipe(pipe) => println!("{:?}", pipe),
    //         //     _ => (),
    //         // }
    //         match tile {
    //             Tile::Animal(ref pipe) => walker = switch_loc(walker, pipe.clone()),
    //             Tile::Pipe(ref pipe) => {
    //                 if pipe.heading.is_none() {
    //                     match walker {
    //                         Loc::Inside => {
    //                             *tile = Tile::Nil(Loc::Inside);
    //                             counter += 1;
    //                         }
    //                         Loc::Outside => *tile = Tile::Nil(Loc::Outside),
    //                     }
    //                 } else {
    //                     walker = switch_loc(walker, pipe.clone())
    //                 }
    //             }
    //             Tile::Nil(_) => match walker {
    //                 Loc::Inside => {
    //                     *tile = Tile::Nil(Loc::Inside);
    //                     counter += 1;
    //                 }
    //                 Loc::Outside => *tile = Tile::Nil(Loc::Outside),
    //             },
    //         }
    //         print!("{}", tile);
    //     }
    //     walker = Loc::Outside;
    //     print!("\n");
    // }
    // println!("{}", counter);
}

// fn switch_loc(current: Loc, pipe: Pipe) -> Loc {
//     if let Some(heading) = pipe.heading {
//         match heading {
//             Face::East => current,
//             Face::North => Loc::Outside,
//             Face::South => Loc::Inside,
//             Face::West => match pipe.first {
//                 Face::West => current,
//                 Face::North => Loc::Inside,
//                 Face::South => Loc::Outside,
//                 _ => unreachable!("Impossible"),
//             },
//         }
//     } else {
//         current
//     }
// }
