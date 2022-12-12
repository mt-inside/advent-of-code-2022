use scanf::sscanf;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let f = File::open("data/5.txt")?;
    let lines = io::BufReader::new(f)
        .lines()
        .flatten() // Remove read errors
        .collect::<Vec<String>>(); // Iterator doesn't have split()

    let sections = lines.split(String::is_empty).collect::<Vec<&[String]>>();

    let mut ts = Towers::new(sections[0])?;
    println!("{}", ts);

    let instrs = sections[1];
    // FIXME: use clap, have --over-9000 flag
    instrs
        .iter()
        .map(|l| l.parse::<Instr>().unwrap())
        .for_each(|i| ts.apply(i, true)); // TODO fold / reduce

    println!("{}", ts);

    println!("{:?}", ts.tops());

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum TowersParseError {
    #[error("Can't determine width")]
    WidthError(#[from] std::num::ParseIntError),
}

#[derive(Debug)]
struct Towers {
    // TODO: interior mutability for towers, so we don't have to take mutable bindings to the whole struct. Then re-write apply loop body as one-liner
    towers: Vec<Vec<char>>,
    width: usize,
}
impl Towers {
    fn new(text: &[String]) -> Result<Self, TowersParseError> {
        let mut towers = text.iter().collect::<VecDeque<&String>>(); // TODO to_vec_deque()?
        let indecies = towers.split_off(towers.len() - 1);
        println!("Towers: {:?}", towers);

        let width = indecies[0]
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .pop()
            .unwrap()
            .parse::<usize>()?;
        println!("Width: {}", width);

        let mut ts = vec![Vec::<char>::new(); width];
        // TODO: feels like it should be a fold / reduce
        towers.iter().rev().map(|l| l.as_bytes()).for_each(|l| {
            (0..width)
                .map(|i| (i, l[i * 4 + 1] as char))
                .filter(|(_, c)| !c.is_ascii_whitespace())
                .for_each(|(i, c)| {
                    ts[i].push(c);
                })
        });

        Ok(Towers { towers: ts, width })
    }

    // TODO: pre-calc & cache
    fn height(&self) -> usize {
        self.towers.iter().map(Vec::len).max().unwrap()
    }

    fn apply(&mut self, instr: Instr, bulk_move: bool) -> () {
        println!("{}", self);
        println!("instr: {}", instr);
        if !bulk_move {
            // 5.1: no bulk moving
            for _ in 0..instr.n {
                let item = self.towers[instr.src].pop().unwrap();
                self.towers[instr.dst].push(item);
            }
        } else {
            // 5.2: bulk moving
            let src_height = self.towers[instr.src].len();
            let mut chunk = self.towers[instr.src].split_off(src_height - instr.n);
            self.towers[instr.dst].append(&mut chunk)
        }
    }

    fn tops(&self) -> String {
        self.towers
            .iter()
            .map(|t| &t[t.len() - 1])
            .collect::<String>()
    }
}
impl std::fmt::Display for Towers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..self.height()).rev() {
            for x in 0..self.width {
                write!(f, "{} ", self.towers[x].get(y).unwrap_or(&' '))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum InstrParseError {
    #[error("shrug")]
    Meh(#[from] std::io::Error),
}

#[derive(Debug)]
struct Instr {
    n: usize,
    src: usize,
    dst: usize,
}
impl std::str::FromStr for Instr {
    type Err = InstrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n: usize = 0;
        let mut src: usize = 0;
        let mut dst: usize = 0;
        sscanf!(s, "move {} from {} to {}", n, src, dst)?;
        Ok(Instr {
            n: n,
            src: src - 1,
            dst: dst - 1,
        })
    }
}
impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.n,
            self.src + 1,
            self.dst + 1
        )
    }
}
