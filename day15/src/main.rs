use std::{
    collections::{hash_map::Iter, HashMap},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let root_dir = env::current_dir().expect("No root?");
    let args = env::args().collect::<Vec<_>>();

    // let file_path = root_dir.join("src").join("sample-2.txt");
    let file_path = root_dir.join("src").join("input.txt");

    let mut line_reader = Line::new(file_path).unwrap();

    if let Some(sequence) = line_reader.next() {
        if let Some(arg) = args.get(1) {
            match arg.as_str() {
                "1" => part_one(sequence),
                "2" => part_two(sequence),
                _ => (),
            }
        }
    } else {
        panic!("Cannot read the sequnce...");
    }
}

struct Line {
    reader: BufReader<File>,
}

impl Line {
    fn new(file_path: PathBuf) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        Ok(Line { reader })
    }
}

impl Iterator for Line {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
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

// ##################################################################################

fn iteration_one(parts: &[&str]) -> u64 {
    if let Some(&first) = parts.get(0) {
        let val = hash(0, first) as u64;
        println!("{} = {}", first, val);
        val + iteration_one(&parts[1..])
    } else {
        0
    }
}

fn hash(current: u16, part: &str) -> u16 {
    if let Some(last) = part.chars().nth(0) {
        let new_value = current + convert_ascii(last);
        let new_value = increase(new_value);
        let new_value = remainder(new_value);
        hash(new_value, &part[1..])
    } else {
        current
    }
}

fn convert_ascii(ch: char) -> u16 {
    (ch as u8) as u16
}

fn increase(current: u16) -> u16 {
    current * 17
}

fn remainder(current: u16) -> u16 {
    current % 256
}

fn part_one(input: String) {
    let binding = input.split(',').collect::<Vec<_>>();
    let parts = binding.as_slice();
    let total = iteration_one(parts);
    println!("Total: {}", total);
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal: u8,
}

impl Lens {
    fn new(label: &str, focal: u8) -> Self {
        Lens {
            label: label.to_string(),
            focal,
        }
    }
}

impl std::fmt::Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal)
    }
}

fn remove_lens(input: &str, mut boxes: HashMap<u8, Vec<Lens>>) -> HashMap<u8, Vec<Lens>> {
    let len = input.len();
    let label = &input[..len - 1];
    let h = hash(0, label) as u8;
    if let Some(lenses) = boxes.get(&h) {
        let lenses_updated = lenses
            .iter()
            .cloned()
            .filter(|l| l.label != label)
            .collect::<Vec<_>>();
        if lenses_updated.len() == 0 {
            boxes.remove(&h);
        } else {
            boxes.insert(h, lenses_updated);
        }
    }
    boxes
}

fn update_lens(input: &str, mut boxes: HashMap<u8, Vec<Lens>>) -> HashMap<u8, Vec<Lens>> {
    let splitted = input.split('=').collect::<Vec<_>>();
    let label = splitted[0];
    let focal = splitted[1].parse::<u8>().expect("failed to parse 😭");

    let h = hash(0, label) as u8;
    if let Some(lenses) = boxes.get(&h) {
        match lenses.iter().find(|&l| l.label == label) {
            Some(_) => {
                let lenses_updated = lenses
                    .iter()
                    .cloned()
                    .map(|l| {
                        if l.label == label {
                            Lens::new(label, focal)
                        } else {
                            l
                        }
                    })
                    .collect::<Vec<_>>();
                boxes.insert(h, lenses_updated);
            }
            None => {
                let mut lenses_updated = lenses.clone();
                lenses_updated.push(Lens::new(label, focal));
                boxes.insert(h, lenses_updated);
            }
        }
    } else {
        let lens = Lens::new(label, focal);
        boxes.insert(h, vec![lens]);
    }
    boxes
}

fn iteration_two(parts: &[&str], boxes: HashMap<u8, Vec<Lens>>) -> HashMap<u8, Vec<Lens>> {
    if let Some(&first) = parts.get(0) {
        if first.contains('=') {
            let updated = update_lens(first, boxes);
            iteration_two(&parts[1..], updated)
        } else {
            let updated = remove_lens(first, boxes);
            iteration_two(&parts[1..], updated)
        }
    } else {
        boxes
    }
}

fn power_lenses(box_num: &u8, lenses: &[Lens], slot: u64) -> u64 {
    if let Some(lens) = lenses.get(0) {
        let p = (*box_num as u64 + 1) * slot * lens.focal as u64;
        println!("{} = {}", lens, p);
        p + power_lenses(box_num, &lenses[1..], slot + 1)
    } else {
        0
    }
}

fn power(mut boxes: Iter<u8, Vec<Lens>>) -> u64 {
    if let Some((box_num, lenses)) = boxes.next() {
        power_lenses(box_num, lenses, 1) + power(boxes)
    } else {
        0
    }
}

fn part_two(input: String) {
    let binding = input.split(',').collect::<Vec<_>>();
    let parts = binding.as_slice();
    let boxes = iteration_two(parts, HashMap::new());
    let p = power(boxes.iter());
    println!("power: {}", p);
}
