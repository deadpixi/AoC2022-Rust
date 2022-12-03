use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Tie => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Desired {
    Win,
    Loss,
    Tie,
}

impl Desired {
    fn from_letter(letter: &str) -> Option<Self> {
        match letter {
            "X" => Some(Self::Loss),
            "Y" => Some(Self::Tie),
            "Z" => Some(Self::Win),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_letter(letter: &str) -> Option<Self> {
        match letter {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissors),
            _ => None,
        }
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn against_desired(&self, desired: Desired) -> Self {
        match (self, desired) {
            (Self::Rock, Desired::Win) => Self::Paper,
            (Self::Rock, Desired::Loss) => Self::Scissors,
            (Self::Paper, Desired::Win) => Self::Scissors,
            (Self::Paper, Desired::Loss) => Self::Rock,
            (Self::Scissors, Desired::Win) => Self::Rock,
            (Self::Scissors, Desired::Loss) => Self::Paper,
            (_, Desired::Tie) => *self,
        }
    }

    fn battle(&self, other: Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Rock, Self::Rock) => Outcome::Tie,
            (Self::Paper, Self::Paper) => Outcome::Tie,
            (Self::Scissors, Self::Scissors) => Outcome::Tie,
            (_, _) => Outcome::Loss,
        }
    }
}

fn problem1(file: File) -> io::Result<()> {
    let mut score = 0usize;

	// The filter_map removes invalid lines.
    for line in io::BufReader::new(file).lines() {
        if let [op, me] = line?
            .trim()
            .split(' ')
            .filter_map(Move::from_letter)
            .collect::<Vec<_>>()
            .as_slice()
        {
            score += me.score() + me.battle(*op).score();
        }
    }

    println!("Problem 1 Score: {}", score);
    Ok(())
}

fn problem2(file: File) -> io::Result<()> {
    let mut score = 0usize;

    for line in io::BufReader::new(file).lines() {
		// This lovely nesting means we just skip over invalid lines.
		// There aren't any invalid lines, but can you imagine?
        if let [op, desired] = line?.trim().split(' ').collect::<Vec<_>>().as_slice() {
            if let Some(op) = Move::from_letter(op) {
                if let Some(desired) = Desired::from_letter(desired) {
                    let me = op.against_desired(desired);
                    score += me.score() + me.battle(op).score();
                }
            }
        }
    }

    println!("Problem 2 Score: {}", score);
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
