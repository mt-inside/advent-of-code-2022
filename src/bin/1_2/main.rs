use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/1.txt")?;
    let mut elves = io::BufReader::new(f)
        .lines()
        .flatten() // Remove read errors
        .collect::<Vec<String>>() // Iterator doesn't have split()
        .split(String::is_empty)
        .map(|ls| {
            ls.iter()
                .map(|l| l.parse::<i64>().expect("parse error"))
                .sum()
        })
        .collect::<Vec<i64>>();
    // Seems there's no .sorted()
    elves.sort_unstable_by(|a, b| b.cmp(a));
    let sum = elves.iter().take(3).sum::<i64>();

    println!("Greediest three elves have {} cals", sum);

    Ok(())
}
