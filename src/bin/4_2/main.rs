use advent_of_code_2022::four::AssignmentPair;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/4.txt")?;
    let c = io::BufReader::new(f)
        .lines()
        .flatten()
        // could parse then flatten, but we want to explode if anything doesn't parse
        .map(|l| l.parse::<AssignmentPair>().unwrap())
        .filter(|ap| ap.overlaps())
        .count();

    println!("Completely containing assignments: {}", c);

    Ok(())
}
