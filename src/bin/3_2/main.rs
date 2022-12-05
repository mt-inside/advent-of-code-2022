#![feature(iter_array_chunks)]

use advent_of_code_2022::intersections;
use advent_of_code_2022::three::pri;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/3.txt")?;
    let res = io::BufReader::new(f)
        .lines()
        .flatten()
        .array_chunks::<3>()
        .map(|chunk| {
            let sets: [HashSet<char>; 3] =
                chunk.map(|rucksack| HashSet::from_iter(rucksack.chars().into_iter()));
            let foo = intersections(&sets[..]);
            let badge = foo.iter().next().unwrap();
            pri(badge)
        })
        .sum::<u32>();

    println!("Sum of priorities: {}", res);

    Ok(())
}
