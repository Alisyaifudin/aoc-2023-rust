use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let root_dir = env::current_dir().expect("No root?");
    let args = env::args().collect::<Vec<_>>();

    // let file_path = root_dir.join("src").join("sample-1.txt");
    let file_path = root_dir.join("src").join("input.txt");

    let puzzle = read_file(file_path);

    for row in &puzzle {
        for tile in row {
            print!("{}", tile)
        }
        println!();
    }

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "1" => part_one(puzzle),
            "2" => part_two(puzzle),
            _ => (),
        }
    }
}

struct Line {
    reader: BufReader<File>,
}

impl Line {
    fn new(path: PathBuf) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Line { reader })
    }
    fn iter(self) -> IterLine {
        IterLine(Line {
            reader: self.reader,
        })
    }
}

struct IterLine(Line);

impl Iterator for IterLine {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.0.reader.read_line(&mut line) {
            Err(e) => panic!("Something wrong {}", e),
            Ok(0) => None,
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop();
                }
                Some(line)
            }
        }
    }
}

enum Tile {
    MirrorForward,   // /
    MirrorBack,      // \
    SplitVertical,   // |
    SplitHorizontal, // -
    Empty,           // .
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::SplitVertical => write!(f, "|"),
            Tile::SplitHorizontal => write!(f, "-"),
            Tile::MirrorBack => write!(f, "\\"),
            Tile::MirrorForward => write!(f, "/"),
        }
    }
}

type Puzzle = Vec<Vec<Tile>>;

fn read_file(path: PathBuf) -> Puzzle {
    let line_reader = Line::new(path).unwrap();
    let mut puzzle = Vec::new();
    let mut current_row = Vec::new();
    for row in line_reader.iter() {
        for ch in row.chars() {
            let tile = match ch {
                '.' => Tile::Empty,
                '/' => Tile::MirrorForward,
                '\\' => Tile::MirrorBack,
                '|' => Tile::SplitVertical,
                '-' => Tile::SplitHorizontal,
                _ => unreachable!("impossible 😡"),
            };
            current_row.push(tile);
        }
        puzzle.push(std::mem::take(&mut current_row));
    }
    puzzle
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Walker {
    x: usize,
    y: usize,
    dir: Dir,
}

impl Walker {
    fn new(x: usize, y: usize, dir: Dir) -> Self {
        Walker { x, y, dir }
    }
}

fn update(walker: &mut Walker, path: &mut HashSet<Walker>) -> (bool, Option<Walker>) {
    if path.get(walker).is_some() {
        return (true, None);
    }
    path.insert(walker.clone());
    (false, None)
}

fn to_down(
    walker: &mut Walker,
    puzzle: &Puzzle,
    y_len: usize,
    path: &mut HashSet<Walker>,
    //   remove?, new?
) -> (bool, Option<Walker>) {
    if walker.y + 1 >= y_len {
        return (true, None);
    }
    walker.y += 1;
    let tile = &puzzle[walker.y][walker.x];
    match tile {
        Tile::MirrorBack => {
            walker.dir = Dir::Right;
            update(walker, path)
        }
        Tile::MirrorForward => {
            walker.dir = Dir::Left;
            update(walker, path)
        }
        Tile::SplitHorizontal => {
            walker.dir = Dir::Right;
            let new = Walker::new(walker.x, walker.y, Dir::Left);
            path.insert(new.clone());
            (update(walker, path).0, Some(new))
        }
        _ => update(walker, path),
    }
}

fn to_left(
    walker: &mut Walker,
    puzzle: &Puzzle,
    path: &mut HashSet<Walker>,
    //   remove?, new?
) -> (bool, Option<Walker>) {
    if walker.x == 0 {
        return (true, None);
    }
    walker.x -= 1;
    let tile = &puzzle[walker.y][walker.x];
    match tile {
        Tile::MirrorBack => {
            walker.dir = Dir::Up;
            update(walker, path)
        }
        Tile::MirrorForward => {
            walker.dir = Dir::Down;
            update(walker, path)
        }
        Tile::SplitVertical => {
            walker.dir = Dir::Up;
            let new = Walker::new(walker.x, walker.y, Dir::Down);
            path.insert(new.clone());
            (update(walker, path).0, Some(new))
        }
        _ => update(walker, path),
    }
}

fn to_up(
    walker: &mut Walker,
    puzzle: &Puzzle,
    path: &mut HashSet<Walker>,
    //   remove?, new?
) -> (bool, Option<Walker>) {
    if walker.y == 0 {
        return (true, None);
    }
    walker.y -= 1;
    let tile = &puzzle[walker.y][walker.x];
    match tile {
        Tile::MirrorBack => {
            walker.dir = Dir::Left;
            update(walker, path)
        }
        Tile::MirrorForward => {
            walker.dir = Dir::Right;
            update(walker, path)
        }
        Tile::SplitHorizontal => {
            walker.dir = Dir::Left;
            let new = Walker::new(walker.x, walker.y, Dir::Right);
            path.insert(new.clone());
            (update(walker, path).0, Some(new))
        }
        _ => update(walker, path),
    }
}

fn to_right(
    walker: &mut Walker,
    puzzle: &Puzzle,
    x_len: usize,
    path: &mut HashSet<Walker>,
    //   remove?, new?
) -> (bool, Option<Walker>) {
    if walker.x + 1 >= x_len {
        return (true, None);
    }
    walker.x += 1;
    let tile = &puzzle[walker.y][walker.x];
    match tile {
        Tile::MirrorBack => {
            walker.dir = Dir::Down;
            update(walker, path)
        }
        Tile::MirrorForward => {
            walker.dir = Dir::Up;
            update(walker, path)
        }
        Tile::SplitVertical => {
            walker.dir = Dir::Down;
            let new = Walker::new(walker.x, walker.y, Dir::Up);
            path.insert(new.clone());
            (update(walker, path).0, Some(new))
        }
        _ => update(walker, path),
    }
}

fn moves(
    walker: &mut Walker,
    path: &mut HashSet<Walker>,
    puzzle: &Puzzle,
) -> (bool, Option<Walker>) {
    let y_len = puzzle.len();
    let x_len = puzzle[0].len();
    match walker.dir {
        Dir::Down => to_down(walker, puzzle, y_len, path),
        Dir::Left => to_left(walker, puzzle, path),
        Dir::Up => to_up(walker, puzzle, path),
        Dir::Right => to_right(walker, puzzle, x_len, path),
    }
}

fn init(walker: Walker, tile: &Tile) -> (Walker, Option<Walker>) {
    let mut initial_walker = walker.clone();
    let mut additional: Option<Walker> = None;
    match walker.dir {
        Dir::Right => match tile {
            Tile::MirrorBack => {
                initial_walker.dir = Dir::Down;
            }
            Tile::MirrorForward => {
                initial_walker.dir = Dir::Up;
            }
            Tile::SplitVertical => {
                initial_walker.dir = Dir::Down;
                additional = Some(Walker::new(walker.x, walker.y, Dir::Up));
            }
            _ => (),
        },
        Dir::Down => match tile {
            Tile::MirrorBack => {
                initial_walker.dir = Dir::Right;
            }
            Tile::MirrorForward => {
                initial_walker.dir = Dir::Left;
            }
            Tile::SplitHorizontal => {
                initial_walker.dir = Dir::Right;
                additional = Some(Walker::new(walker.x, walker.y, Dir::Left));
            }
            _ => (),
        },
        Dir::Left => match tile {
            Tile::MirrorBack => {
                initial_walker.dir = Dir::Up;
            }
            Tile::MirrorForward => {
                initial_walker.dir = Dir::Down;
            }
            Tile::SplitVertical => {
                initial_walker.dir = Dir::Up;
                additional = Some(Walker::new(walker.x, walker.y, Dir::Down));
            }
            _ => (),
        },
        Dir::Up => match tile {
            Tile::MirrorBack => {
                initial_walker.dir = Dir::Left;
            }
            Tile::MirrorForward => {
                initial_walker.dir = Dir::Right;
            }
            Tile::SplitHorizontal => {
                initial_walker.dir = Dir::Left;
                additional = Some(Walker::new(walker.x, walker.y, Dir::Right));
            }
            _ => (),
        },
    }

    (initial_walker, additional)
}

fn walk(walkers: &mut Vec<Walker>, path: &mut HashSet<Walker>, puzzle: &Puzzle) {
    for i in (0..walkers.len()).rev() {
        let (removed, new) = moves(&mut walkers[i], path, puzzle);
        if removed {
            walkers.remove(i);
        }
        if let Some(w) = new {
            walkers.push(w);
        }
    }
}

fn laser(puzzle: &Puzzle, first: Walker, second: Option<Walker>) -> (usize, HashSet<(usize, usize)>) {
    let mut walkers = vec![first];
    let mut path = HashSet::new();
    path.insert(walkers[0].clone());
    if let Some(s) = second {
        walkers.push(s);
        path.insert(walkers[1].clone());
    }
    loop {
        if walkers.len() == 0 {
            break;
        }
        walk(&mut walkers, &mut path, &puzzle);
    }
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    for walker in path.iter() {
        let value = (walker.x, walker.y);
        tiles.insert(value);
    }

    let total = tiles.len();
    (total, tiles)
}

fn part_one(puzzle: Puzzle) {
    let (first, second) = init(Walker::new(0, 0, Dir::Right), &puzzle[0][0]);
    let (total, tiles) = laser(&puzzle, first, second);

    println!();
    for (y, row) in puzzle.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tiles.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }

    println!("Total: {}", total);
}

fn part_two(puzzle: Puzzle) {
    let mut highest = (0, HashSet::new());
    //first row
    for (x, tile) in puzzle[0].iter().enumerate() {
        let (first, second) = init(Walker::new(x, 0, Dir::Down), tile);
        let (total, tiles) = laser(&puzzle, first, second);
        if total > highest.0 {
            highest = (total, tiles);
        }
    }
    //last row
    let y_len = puzzle.len();
    for (x, tile) in puzzle[y_len - 1].iter().enumerate() {
        let (first, second) = init(Walker::new(x, y_len - 1, Dir::Up), tile);
        let (total, tiles) = laser(&puzzle, first, second);
        if total > highest.0 {
            highest = (total, tiles);
        }
    }
    //first column
    for y in 0..puzzle.len() {
        let tile = &puzzle[y][0];
        let (first, second) = init(Walker::new(0, y, Dir::Right), tile);
        let (total, tiles) = laser(&puzzle, first, second);
        if total > highest.0 {
            highest = (total, tiles);
        }
    }
    //last column
    let x_len = puzzle[0].len();
    for y in 0..puzzle.len() {
        let tile = &puzzle[y][x_len - 1];
        let (first, second) = init(Walker::new(x_len - 1, y, Dir::Left), tile);
        let (total, tiles) = laser(&puzzle, first, second);
        if total > highest.0 {
            highest = (total, tiles);
        }
    }

    // print path
    println!();
    for (y, row) in puzzle.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if highest.1.get(&(x, y)).is_some() {
                print!("#");
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }

    println!("Highest: {}", highest.0);
}
