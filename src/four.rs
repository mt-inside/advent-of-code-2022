use crate::lift_m2;
use std::num::ParseIntError;

pub struct Range {
    min: u32,
    max: u32,
}
impl Range {
    fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
    pub fn contains(&self, other: &Range) -> bool {
        other.min >= self.min && other.max <= self.max
    }
    pub fn overlaps(&self, other: &Range) -> bool {
        other.min <= self.max && other.max >= self.min
    }
}
impl std::str::FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("-").collect::<Vec<&str>>();
        lift_m2(Range::new, parts[0].parse(), parts[1].parse())
    }
}

pub struct AssignmentPair {
    one: Range,
    two: Range,
}
impl AssignmentPair {
    fn new(one: Range, two: Range) -> Self {
        Self { one, two }
    }

    pub fn contains(&self) -> bool {
        self.one.contains(&self.two) || self.two.contains(&self.one)
    }
    pub fn overlaps(&self) -> bool {
        self.one.overlaps(&self.two) || self.two.overlaps(&self.one)
    }
}
impl std::str::FromStr for AssignmentPair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(",").collect::<Vec<&str>>();
        lift_m2(AssignmentPair::new, parts[0].parse(), parts[1].parse())
    }
}
