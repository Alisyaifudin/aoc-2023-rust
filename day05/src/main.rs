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
            Ok(_) => Ok(Some(line)),
            Err(e) => Err(e),
        }
    }
}

// function

fn read_file(mut line_reader: Line) -> Result<(Vec<usize>, Vec<Almanac>), Error<'static>> {
    // read first line
    let first_line = line_reader.read().map_err(Error::Io)?;
    let seeds = match first_line {
        Some(line) => {
            let splitted = line.split(':').collect::<Vec<&str>>();
            if splitted.len() != 2 {
                panic!("Invalid first line")
            }
            let seeds = splitted[1]
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            seeds
        }
        None => panic!("No first line found"),
    };

    let mut almanacs = Vec::new();
    let mut current_almanac = Almanac::new();

    let _ = line_reader.read(); // skip second line

    loop {
        match line_reader.read() {
            Ok(Some(line)) => {
                if line == "\n" {
                    current_almanac.sort_by_source();
                    almanacs.push(current_almanac.clone());
                    current_almanac.clear();
                    continue;
                }
                if line.contains("map") {
                    continue;
                }
                let splitted = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                if splitted.len() != 3 {
                    panic!("Invalid line")
                }
                current_almanac.add(Map::new(&splitted[0], &splitted[1], &splitted[2]));
            }
            Ok(None) => {
                // End of file reached
                current_almanac.sort_by_source();
                almanacs.push(current_almanac.clone());
                current_almanac.clear();
                break;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(Error::Io(e));
            }
        }
    }
    Ok((seeds, almanacs))
}

// #################################################################################
// #################################################################################
// #################################################################################

// struct

#[derive(Debug, Clone)]
struct Map {
    start_destination: usize,
    start_source: usize,
    len: usize,
}

impl Map {
    fn new(start_destination: &usize, start_source: &usize, len: &usize) -> Self {
        Map {
            start_destination: start_destination.clone(),
            start_source: start_source.clone(),
            len: len.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    map: Vec<Map>,
}

impl Almanac {
    fn new() -> Self {
        Almanac { map: Vec::new() }
    }
    fn add(&mut self, map: Map) {
        self.map.push(map)
    }
    fn sort_by_source(&mut self) {
        self.map.sort_by(|a, b| a.start_source.cmp(&b.start_source));
    }
    fn convert(&mut self, source: usize) -> usize {
        for map in &self.map {
            if source < map.start_source {
                return source;
            }
            if source < map.start_source + map.len {
                let delta = source - map.start_source;
                return map.start_destination + delta;
            }
        }
        source
    }
    fn clear(&mut self) {
        self.map.clear();
    }
    fn source_bands(&self) -> Vec<Band> {
        let mut bands = Vec::new();
        for map in &self.map {
            bands.push(Band::new(map.start_source, map.len))
        }
        bands
    }
}

#[derive(Debug, Clone)]
struct Band {
    start: usize,
    end: usize,
}

impl Band {
    fn new(start: usize, len: usize) -> Self {
        Band {
            start,
            end: start + len - 1,
        }
    }
    fn from_boundaries(start: usize, end: usize) -> Self {
        Band { start, end }
    }
    // fn combine(&self, other: &Band) -> Option<Band> {
    //     if self.end < other.start || other.end < self.start {
    //         return None;
    //     }
    //     let start = self.start.min(other.start);
    //     let end = self.end.max(other.end);
    //     Some(Band { start, end })
    // }
    // fn union(bands: &Vec<Band>) -> Vec<Band> {
    //     let mut all = bands.clone();
    //     'outer: loop {
    //         let num = all.len();
    //         'comp: for i in 0..(num - 1) {
    //             for j in (i + 1)..num {
    //                 let combined = all[i].combine(&all[j]);
    //                 if let Some(com) = combined {
    //                     all.remove(j);
    //                     all.remove(i);
    //                     all.push(com);
    //                     break 'comp;
    //                 }
    //             }
    //             if i == num - 1 {
    //                 break 'outer;
    //             }
    //         }
    //     }
    //     all
    // }
}

// ###################################################################################
// main
// part_one
fn part_one(line_reader: Line) -> Result<(), Error<'static>> {
    let (seeds, mut almanacs) = read_file(line_reader)?;

    let mut target = seeds;

    for almanac in almanacs.iter_mut() {
        for i in 0..target.len() {
            target[i] = almanac.convert(target[i]);
        }
    }
    let min = target.iter().min().expect("No minimum?");

    println!("min: {}", min);
    Ok(())
}

// part_two

fn chopped(ori: &Band, target: &Band) -> Option<Vec<Band>> {
    // disjoint
    if ori.end < target.start || target.end < ori.start {
        return None;
    }
    // ori inside
    if ori.start >= target.start && ori.end <= target.end {
        return None;
    }
    // target inside
    if ori.start < target.start && ori.end > target.end {
        let above_band = Band::from_boundaries(target.end + 1, ori.end);
        let middle_band = Band::from_boundaries(target.start, target.end);
        let below_band = Band::from_boundaries(ori.start, target.start - 1);
        return Some(vec![above_band, middle_band, below_band]);
    }
    // partly below
    if target.start <= ori.start && ori.end >= target.end {
        let above_band = Band::from_boundaries(target.end + 1, ori.end);
        let below_band = Band::from_boundaries(ori.start, target.end);
        return Some(vec![above_band, below_band]);
    }
    // partly above
    if target.start >= ori.start && ori.end <= target.end {
        let above_band = Band::from_boundaries(target.start, ori.end);
        let below_band = Band::from_boundaries(ori.start, target.start - 1);
        return Some(vec![above_band, below_band]);
    }
    eprintln!("ori: {:?}", ori);
    eprintln!("target: {:?}", target);
    unreachable!("No other possibility");
}

fn part_two(line_reader: Line) -> Result<(), Error<'static>> {
    let (seeds, mut almanacs) = read_file(line_reader)?;
    let mut bands = Vec::new();
    for chunks in seeds.chunks_exact(2) {
        let start = chunks[0];
        let length = chunks[1];
        bands.push(Band::new(start, length));
    }
    let mut additional = Vec::new();
    let mut indices = Vec::new();
    let mut next_bands = bands;
    for almanac in almanacs.iter_mut() {
        bands = next_bands.clone();
        next_bands.clear();
        let source_bands = almanac.source_bands();
        // chopping the input bands
        for source_band in &source_bands {
            for (i, band) in bands.iter().enumerate() {
                let splitted = chopped(band, source_band);
                if let Some(s) = splitted {
                    additional.extend(s);
                    indices.push(i);
                }
            }
            for i in indices.iter().rev() {
                bands.remove(i.clone());
            }
            bands.extend(additional.clone());
            additional.clear();
            indices.clear();
        }
        // convert to the next bands
        for band in &bands {
            let start = almanac.convert(band.start);
            let end = almanac.convert(band.end);
            next_bands.push(Band::from_boundaries(start, end));
        }
    }
    let min = next_bands
        .iter()
        .map(|b| b.start)
        .min()
        .expect("No minimum?");
    println!("min: {}", min);
    Ok(())
}
