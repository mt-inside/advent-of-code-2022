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
        .map(|rucksack| HashSet::from_iter(rucksack.chars().into_iter()))
        .array_chunks::<3>()
        .map(|chunk| {
            let commons = intersections(&chunk[..]);
            let badge = commons.iter().next().unwrap();
            pri(badge)
        })
        .sum::<u32>();

    println!("Sum of priorities: {}", res);

    Ok(())
}
