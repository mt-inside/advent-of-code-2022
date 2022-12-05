use advent_of_code_2022::three::pri;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/3.txt")?;
    let lines = io::BufReader::new(f)
        .lines()
        .flatten()
        .collect::<Vec<String>>();

    let res = calc(lines);
    println!("Sum of priorities: {}", res);

    Ok(())
}

fn calc(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|l| {
            // len() is byte-count, not utf-8 char count (which would
            // be .chars().count()), but input is guarenteed to be
            // ascii, and len() is O(1) vs O(n).
            let (one, two) = l.split_at(l.len() / 2);
            // don't know why this needs a type hint
            let s1: HashSet<_, RandomState> = HashSet::from_iter(one.chars().into_iter());
            let s2 = HashSet::from_iter(two.chars().into_iter());
            let mut pair = s1.intersection(&s2);
            pri(pair.next().unwrap())
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(calc(vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_owned()]), 16);

        assert_eq!(
            calc(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_owned(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_owned(),
                "PmmdzqPrVvPwwTWBwg".to_owned(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned(),
                "ttgJtRGJQctTZtZT".to_owned(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_owned()
            ]),
            157
        );
    }
}
