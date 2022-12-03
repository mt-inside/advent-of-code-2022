use advent_of_code_2022::two::Round;
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
        .map(|r: Round| r.score())
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_outcome() {
        use Outcome::*;

        assert_eq!(
            outcome(&Round {
                me: Move::Paper,
                them: Move::Paper,
            }),
            Draw
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Paper,
                them: Move::Scissors,
            }),
            Loss,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Paper,
                them: Move::Rock,
            }),
            Win,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Scissors,
                them: Move::Paper,
            }),
            Win,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Scissors,
                them: Move::Scissors,
            }),
            Draw,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Scissors,
                them: Move::Rock,
            }),
            Loss,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Rock,
                them: Move::Paper
            }),
            Loss,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Rock,
                them: Move::Scissors,
            }),
            Win,
        );
        assert_eq!(
            outcome(&Round {
                me: Move::Rock,
                them: Move::Rock,
            }),
            Draw,
        );
    }

    #[test]
    fn test_score_round() {
        assert_eq!(
            score_round(&Round {
                me: Move::Paper,
                them: Move::Paper,
            }),
            5
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Paper,
                them: Move::Scissors,
            }),
            2
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Paper,
                them: Move::Rock,
            }),
            8
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Scissors,
                them: Move::Paper,
            }),
            9
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Scissors,
                them: Move::Scissors,
            }),
            6
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Scissors,
                them: Move::Rock,
            }),
            3
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Rock,
                them: Move::Paper
            }),
            1,
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Rock,
                them: Move::Scissors,
            }),
            7
        );
        assert_eq!(
            score_round(&Round {
                me: Move::Rock,
                them: Move::Rock,
            }),
            4
        );
    }

    #[test]
    fn test_parse_and_sum_single() {
        assert_eq!(
            parse_and_sum(Box::new(vec!("A X".to_owned()).into_iter())),
            4
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("A Y".to_owned()).into_iter())),
            8
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("A Z".to_owned()).into_iter())),
            3
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("B X".to_owned()).into_iter())),
            1
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("B Y".to_owned()).into_iter())),
            5
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("B Z".to_owned()).into_iter())),
            9
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("C X".to_owned()).into_iter())),
            7
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("C Y".to_owned()).into_iter())),
            2
        );
        assert_eq!(
            parse_and_sum(Box::new(vec!("C Z".to_owned()).into_iter())),
            6
        );
    }

    #[test]
    fn test_parse_and_sum_multi() {
        assert_eq!(
            parse_and_sum(Box::new(
                vec!("A X".to_owned(), "A X".to_owned()).into_iter() // Rock, Rock
            )),
            8
        );
        assert_eq!(
            parse_and_sum(Box::new(
                vec!(
                    "C Y".to_owned(), // Scissors, Paper
                    "C Y".to_owned(), // Scissors, Paper
                    "C Y".to_owned(), // Scissors, Paper
                    "B Z".to_owned(), // Paper, Scissors
                )
                .into_iter()
            )),
            15
        );
    }
}
