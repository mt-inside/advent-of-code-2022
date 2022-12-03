use advent_of_code_2022::two::{Round, Thrower};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/2.txt")?;
    let ls = io::BufReader::new(f).lines().flatten();

    let sum = parse_and_sum(Box::new(ls));
    println!("Total score: {}", sum);

    Ok(())
}

fn parse_and_sum(ls: Box<dyn Iterator<Item = String>>) -> i32 {
    ls.map(|l| l.parse().unwrap())
        .map(|t: Thrower| t.throw()) // TODO: Thrower::throw doesn't work - borrow-ness issues
        .map(|r: Round| r.score())
        .sum::<i32>()
}
