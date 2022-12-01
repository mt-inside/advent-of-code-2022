use std::fs::File;
use std::io::{self, BufRead};

/* Mutable code for the algorithmics.
 * Puzzle 1.2 done immutably */

fn main() -> anyhow::Result<()> {
    let f = File::open("data/1.txt")?;
    let mut max = 0;
    let mut cur = 0;
    io::BufReader::new(f)
        .lines()
        .for_each(|l| match l.expect("Failed to read line").as_ref() {
            "" => {
                if cur > max {
                    max = cur;
                }
                cur = 0;
            }
            a => {
                let n = a.parse::<i64>().unwrap();
                cur += n;
            }
        });

    println!("Greediest reindeer has {} cals", max);

    Ok(())
}
