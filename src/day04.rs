use aoc::args::Puzzle;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Assign {
    start: u32,
    end: u32,
}

impl Assign {
    #[cfg(test)]
    pub fn new(start: u32, end: u32) -> Assign {
        Assign { start, end }
    }

    pub fn contains_section(&self, num: u32) -> bool {
        self.start <= num && num <= self.end
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.contains_section(other.start) && self.contains_section(other.end)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.contains_section(other.start) || self.contains_section(other.end)
            || other.contains(self)
    }
}

impl FromStr for Assign {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.split('-').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("Invalid number of parts in range");
        }

        Ok(Self {
            start: parts[0].parse::<u32>().expect("Cannot parse range start"),
            end: parts[1].parse::<u32>().expect("Cannot parse range end"),
        })
    }
}

#[derive(Debug)]
struct AssignPair {
    elves: [Assign; 2],
}

impl AssignPair {
    fn has_complete_overlap(&self) -> bool {
        self.elves[0].contains(&self.elves[1])
            || self.elves[1].contains(&self.elves[0])
    }

    fn has_overlap(&self) -> bool {
        self.elves[0].overlaps(&self.elves[1])
    }
}

impl FromStr for AssignPair {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts = value.split(',').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("Invalid number of parts in assignment pair");
        }

        Ok(Self {
            elves: parts.iter()
                .map(|p| p.parse::<Assign>().expect("Cannot parse range"))
                .collect::<Vec<Assign>>()
                .try_into().expect("Invalid number of ranges"),
        })
    }
}

fn count_complete_overlaps(pairs: &[AssignPair]) -> usize {
    pairs.iter().filter(|p| p.has_complete_overlap()).count()
}

fn count_overlaps(pairs: &[AssignPair]) -> usize {
    pairs.iter().filter(|p| p.has_overlap()).count()
}

fn read_pairs(file: &File) -> Result<Vec<AssignPair>, &'static str> {
    let mut lines = BufReader::new(file).lines();
    let mut pairs = Vec::new();

    while let Some(line) = aoc::io::read_line(&mut lines) {
        pairs.push(line.parse()?);
    }

    Ok(pairs)
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let pairs = read_pairs(&file).expect("Cannot read rucksacks");

    match args.puzzle {
        Puzzle::P1 => println!("{}", count_complete_overlaps(&pairs)),
        Puzzle::P2 => println!("{}", count_overlaps(&pairs)),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(s1: u32, e1: u32, s2: u32, e2: u32) -> AssignPair {
        AssignPair {
            elves: [Assign::new(s1, e1), Assign::new(s2, e2)],
        }
    }

    #[test]
    fn contains() {
        assert!(Assign::new(1, 4).contains(&Assign::new(1, 4)));
        assert!(Assign::new(0, 2).contains(&Assign::new(1, 2)));
        assert!(Assign::new(0, 2).contains(&Assign::new(0, 1)));
        assert!(Assign::new(0, 5).contains(&Assign::new(1, 4)));

        assert!(!Assign::new(3, 5).contains(&Assign::new(2, 4)));
        assert!(!Assign::new(3, 5).contains(&Assign::new(4, 6)));
    }

    fn pairs() -> Vec<AssignPair> {
        vec![
            pair(2, 8, 3, 7),
            pair(6, 6, 4, 6),

            pair(2, 4, 6, 8),
            pair(2, 3, 4, 5),
            pair(5, 7, 7, 9),
            pair(2, 6, 4, 8),
        ]
    }

    #[test]
    fn complete_overlaps() {
        let pairs = pairs();
        assert!(pairs[0].has_complete_overlap());
        assert!(pairs[1].has_complete_overlap());

        assert!(!pairs[2].has_complete_overlap());
        assert!(!pairs[3].has_complete_overlap());
        assert!(!pairs[4].has_complete_overlap());
        assert!(!pairs[5].has_complete_overlap());
    }

    #[test]
    fn p1_example() {
        assert_eq!(count_complete_overlaps(&pairs()), 2);
    }

    #[test]
    fn partial_overlaps() {
        let pairs = pairs();
        assert!(pairs[0].has_overlap());
        assert!(pairs[1].has_overlap());
        assert!(pairs[4].has_overlap());
        assert!(pairs[5].has_overlap());

        assert!(!pairs[2].has_overlap());
        assert!(!pairs[3].has_overlap());
    }

    #[test]
    fn p2_example() {
        assert_eq!(count_overlaps(&pairs()), 4);
    }
}
