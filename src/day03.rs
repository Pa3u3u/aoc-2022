use aoc::args::Puzzle;
use std::collections::HashSet as Set;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io::Result as IOResult;

#[derive(Debug)]
struct Rucksack {
    parts: [Set<char>; 2],
}

impl Rucksack {
    fn _check_str(s: &str) -> Result<(), &'static str> {
        if s.len() % 2 != 0 {
            return Err("Invalid length");
        }

        for c in s.chars() {
            if !c.is_ascii_alphabetic() {
                return Err("Invalid character in string");
            }
        }

        Ok(())
    }

    pub fn from_str(s: &str) -> Result<Rucksack, &'static str>  {
        Rucksack::_check_str(s)?;

        let mut rucksack: Rucksack = Rucksack {
            parts: [Set::new(), Set::new()],
        };

        let (l1, l2) = s.split_at(s.len() / 2);
        for (part, split) in std::iter::zip(&mut rucksack.parts, [l1, l2]) {
            part.extend(split.chars());
        }

        Ok(rucksack)
    }

    pub fn common(&self) -> Set<char> {
        self.parts[0].intersection(&self.parts[1]).copied().collect()
    }
}

fn eval_letter(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        return u32::from(*c) - u32::from('a') + 1;
    }

    u32::from(*c) - u32::from('A') + 27
}

fn read_rucksacks(file: &File) -> Result<Vec<Rucksack>, &'static str> {
    let mut lines = BufReader::new(file).lines();
    let mut rucksacks = Vec::new();

    while let Some(line) = aoc::io::read_line(&mut lines) {
        rucksacks.push(Rucksack::from_str(&line)?);
    }

    Ok(rucksacks)
}

fn rucksacks_value(rs: &[Rucksack]) -> u32 {
    rs.iter().map(|r| r.common().iter().map(eval_letter).sum::<u32>()).sum()
}

struct Group<'a> {
    rucksacks: Vec<&'a Rucksack>,
}

fn create_groups(rs: &Vec<Rucksack>) -> Result<Vec<Group>, &'static str> {
    if rs.len() % 3 != 0 {
        return Err("Invalid number of rucksacks");
    }

    let mut result = Vec::new();
    for w in rs.chunks(3) {
        let group = Group {
            rucksacks: w.iter().collect(),
        };

        result.push(group);
    }

    Ok(result)
}

fn find_badge(group: &Group) -> Result<char, &'static str> {
    let common = group.rucksacks.iter()
        .map(|r| r.parts[0].union(&r.parts[1]).copied().collect::<Set<char>>())
        .reduce(|a, m| a.intersection(&m).copied().collect())
        .ok_or("BUG: No intersection found")?;

    if common.len() > 1 {
        return Err("Too many badges found");
    }

    common.iter().next().copied().ok_or("No badge found")
}

fn count_badges(rs: &Vec<Rucksack>) -> Result<u32, &'static str> {
    let groups = create_groups(rs)?;
    let badges = groups.iter().filter_map(|g| find_badge(g).ok())
        .map(|p| eval_letter(&p));

    Ok(badges.sum())
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let rucksacks = read_rucksacks(&file).expect("Cannot read rucksacks");

    match args.puzzle {
        Puzzle::P1 => println!("{}", rucksacks_value(&rucksacks)),
        Puzzle::P2 => println!("{}",
                count_badges(&rucksacks).map_err(|e| Error::new(ErrorKind::Other, e))?),
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn build(s: &str) -> Rucksack {
        Rucksack::from_str(s).expect("Invalid rucksack")
    }

    fn letters(s: &str) -> Set<char> {
        let mut result = Set::new();
        result.extend(s.chars());
        result
    }

    #[test]
    fn eval_letter() {
        assert_eq!(super::eval_letter(&'a'), 1);
        assert_eq!(super::eval_letter(&'z'), 26);
        assert_eq!(super::eval_letter(&'A'), 27);
        assert_eq!(super::eval_letter(&'Z'), 52);
    }

    #[test]
    fn common_chars() {
        assert_eq!(build("vJrwpWtwJgWrhcsFMMfFFhFp").common(), letters("p"));
        assert_eq!(build("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").common(), letters("L"));
        assert_eq!(build("PmmdzqPrVvPwwTWBwg").common(), letters("P"));
        assert_eq!(build("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").common(), letters("v"));
        assert_eq!(build("ttgJtRGJQctTZtZT").common(), letters("t"));
        assert_eq!(build("CrZsJsPPZsGzwwsLwLmpwMDw").common(), letters("s"));
    }

    fn rucksacks() -> Vec<Rucksack> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].iter().map(|s| build(s)).collect::<Vec<Rucksack>>()
    }

    #[test]
    fn p1_total() {
        let rucksacks = rucksacks();
        assert_eq!(rucksacks_value(&rucksacks), 157);
    }

    #[test]
    fn p2_example1() {
        let rucksacks = rucksacks();
        let groups = create_groups(&rucksacks).expect("Cannot create group");
        assert_eq!(groups.len(), 2);
        assert_eq!(find_badge(&groups[0]).expect("find_badge()"), 'r');
        assert_eq!(find_badge(&groups[1]).expect("find_badge()"), 'Z');
    }
}
