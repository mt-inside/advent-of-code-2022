use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/1.txt")?;
    // Seems there's no .sorted()
    let mut elves = io::BufReader::new(f)
        .lines()
        .fold(VecDeque::from([0]), |mut es, l| {
            match l.expect("Failed to read line").as_ref() {
                "" => {
                    es.push_front(0);
                }
                a => {
                    let n = a.parse::<i64>().unwrap();
                    es[0] += n;
                }
            }
            es
        });
    elves.make_contiguous().sort_unstable_by(|a, b| b.cmp(a));
    let sum = elves.iter().take(3).sum::<i64>();

    println!("Greediest three elves have {} cals", sum);

    Ok(())
}
