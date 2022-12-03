use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(c: char) -> u64 {
    (PRIORITIES.find(c).unwrap_or(0) + 1) as u64
}

fn problem1(file: File) -> io::Result<()> {
    let mut total = 0u64;
    for line in io::BufReader::new(file).lines().flatten() {
        total += line[..line.len() / 2]
            .chars()
            .unique()
            .filter(|x| line[line.len() / 2..].contains(*x))
            .map(get_priority)
            .sum::<u64>();
    }

    println!("Problem 1 Score: {}", total);
    Ok(())
}

fn problem2(file: File) -> io::Result<()> {
    let mut total = 0u64;

    for group in io::BufReader::new(file)
        .lines()
        .flatten()
        .chunks(3)
        .into_iter()
    {
        let bags: Vec<String> = group.collect();
        total += bags[0]
            .chars()
            .unique()
            .filter(|x| bags[1].contains(*x) && bags[2].contains(*x))
            .map(get_priority)
            .sum::<u64>();
    }

    println!("Problem 2 Score: {}", total);
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
