use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;

struct InvalidIntervalError {}

impl From<ParseIntError> for InvalidIntervalError {
    fn from(_error: ParseIntError) -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    fn from_string(input: &str) -> Result<Self, InvalidIntervalError> {
        if let [start, end] = input.split('-').collect::<Vec<_>>().as_slice() {
            Ok(Self {
                start: start.parse::<usize>()?,
                end: end.parse::<usize>()?,
            })
        } else {
            Err(InvalidIntervalError {})
        }
    }

    fn overlaps(&self, other: Self) -> bool {
        (other.start >= self.start && other.end <= self.end)
            || (other.end >= self.start && other.end <= self.end)
            || (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }

    fn contains(&self, other: Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }
}

fn problem1(file: File) -> io::Result<()> {
    let total = io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|x| {
            x.split(',')
                .flat_map(Interval::from_string)
                .collect::<Vec<Interval>>()
        })
        .filter(|x| x[0].contains(x[1]) || x[1].contains(x[0]))
        .count();
    println!("Total: {}", total);
    Ok(())
}

fn problem2(file: File) -> io::Result<()> {
    let total = io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|x| {
            x.split(',')
                .flat_map(Interval::from_string)
                .collect::<Vec<Interval>>()
        })
        .filter(|x| x[0].overlaps(x[1]))
        .count();
    println!("Total: {}", total);
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[2])?;
    if args[1] == "1" {
        problem1(file)
    } else {
        problem2(file)
    }
}
