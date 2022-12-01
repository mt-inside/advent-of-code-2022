use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/1.txt")?;
    // Seems there's no .sorted()
    let mut elves = io::BufReader::new(f)
        .lines()
        .flatten() // Remove read errors
        .collect::<Vec<String>>() // Iterator doesn't have split()
        .split(|l| l == "")
        .map(|ls| {
            ls.iter()
                .map(|l| l.parse::<i64>().expect("parse error"))
                .sum()
        })
        .collect::<Vec<i64>>();
    elves.sort_unstable_by(|a, b| b.cmp(a));
    let sum = elves.iter().take(3).sum::<i64>();

    println!("Greediest three elves have {} cals", sum);

    Ok(())
}
