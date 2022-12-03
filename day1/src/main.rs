use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[2])?;

    let mut sum = 0u32;
    let mut top4: [u32; 4] = [0, 0, 0, 0];

    for line in io::BufReader::new(file).lines() {
        if let Ok(n) = line?.parse::<u32>() {
            sum += n;
        } else {
            top4[3] = sum.max(*top4.iter().min().unwrap_or(&0));
            top4.sort_by(|a, b| b.cmp(a));
            sum = 0;
        }
    }

    println!(
        "{}",
        if args[1] == "1" {
            top4.iter()
                .take(3)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            top4[0..=2].iter().sum::<u32>().to_string()
        }
    );

    Ok(())
}
