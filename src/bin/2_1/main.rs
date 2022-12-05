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
        use advent_of_code_2022::two::Move::*;
        use advent_of_code_2022::two::Outcome::*;

        assert_eq!(
            Round {
                me: Paper,
                them: Paper,
            }
            .outcome(),
            Draw
        );
        assert_eq!(
            Round {
                me: Paper,
                them: Scissors,
            }
            .outcome(),
            Loss,
        );
        assert_eq!(
            Round {
                me: Paper,
                them: Rock,
            }
            .outcome(),
            Win,
        );
        assert_eq!(
            Round {
                me: Scissors,
                them: Paper,
            }
            .outcome(),
            Win,
        );
        assert_eq!(
            Round {
                me: Scissors,
                them: Scissors,
            }
            .outcome(),
            Draw,
        );
        assert_eq!(
            Round {
                me: Scissors,
                them: Rock,
            }
            .outcome(),
            Loss,
        );
        assert_eq!(
            Round {
                me: Rock,
                them: Paper
            }
            .outcome(),
            Loss,
        );
        assert_eq!(
            Round {
                me: Rock,
                them: Scissors,
            }
            .outcome(),
            Win,
        );
        assert_eq!(
            Round {
                me: Rock,
                them: Rock,
            }
            .outcome(),
            Draw,
        );
    }

    #[test]
    fn test_score_round() {
        use advent_of_code_2022::two::Move::*;

        assert_eq!(
            Round {
                me: Paper,
                them: Paper,
            }
            .score(),
            5
        );
        assert_eq!(
            Round {
                me: Paper,
                them: Scissors,
            }
            .score(),
            2
        );
        assert_eq!(
            Round {
                me: Paper,
                them: Rock,
            }
            .score(),
            8
        );
        assert_eq!(
            Round {
                me: Scissors,
                them: Paper,
            }
            .score(),
            9
        );
        assert_eq!(
            Round {
                me: Scissors,
                them: Scissors,
            }
            .score(),
            6
        );
        assert_eq!(
            Round {
                me: Scissors,
                them: Rock,
            }
            .score(),
            3
        );
        assert_eq!(
            Round {
                me: Rock,
                them: Paper
            }
            .score(),
            1,
        );
        assert_eq!(
            Round {
                me: Rock,
                them: Scissors,
            }
            .score(),
            7
        );
        assert_eq!(
            Round {
                me: Rock,
                them: Rock,
            }
            .score(),
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
