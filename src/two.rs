use crate::lift_m2;

#[derive(thiserror::Error, Debug)]
pub enum MoveParseError {
    #[error("Move `{0}` not recognised")]
    NotRecognised(String),
}

#[derive(Copy, Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    pub fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
impl std::str::FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::*;

        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err(MoveParseError::NotRecognised(s.to_owned())),
        }
    }
}

pub struct Round {
    pub me: Move,
    pub them: Move,
}
impl Round {
    fn new(me: Move, them: Move) -> Self {
        Round { me, them }
    }
    pub fn score(&self) -> i32 {
        self.me.score() + self.outcome() as i32
    }
    pub fn outcome(&self) -> Outcome {
        use Move::*;
        use Outcome::*;

        match (self.them, self.me) {
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Win,
            (Paper, Rock) => Loss,
            (Scissors, Paper) => Loss,
            (Scissors, Scissors) => Draw,
            (Scissors, Rock) => Win,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Loss,
            (Rock, Rock) => Draw,
        }
    }
}
impl std::str::FromStr for Round {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        lift_m2(Round::new, parts[0].parse(), parts[1].parse())
    }
}

pub struct Thrower {
    them: Move,
    outcome: Outcome,
}
impl Thrower {
    fn new(them: Move, outcome: Outcome) -> Self {
        Thrower { them, outcome }
    }

    pub fn throw(&self) -> Round {
        use Move::*;
        use Outcome::*;

        let mv = match (self.them, self.outcome) {
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
        };
        Round {
            them: self.them,
            me: mv,
        }
    }
}
impl std::str::FromStr for Thrower {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        lift_m2(Thrower::new, parts[0].parse(), parts[1].parse())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}
impl std::str::FromStr for Outcome {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;

        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(MoveParseError::NotRecognised(s.to_owned())),
        }
    }
}
