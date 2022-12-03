use aoc::args::Puzzle;
use std::collections::HashSet as Set;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
        self.parts[0].intersection(&self.parts[1]).map(|x| x.clone()).collect()
    }
}

fn eval_letter(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        return u32::from(*c) - u32::from('a') + 1;
    }

    return u32::from(*c) - u32::from('A') + 27;
}

fn read_rucksacks(file: &File) -> Result<Vec<Rucksack>, &'static str> {
    let mut lines = BufReader::new(file).lines();
    let mut rucksacks = Vec::new();

    while let Some(line) = aoc::io::read_line(&mut lines) {
        rucksacks.push(Rucksack::from_str(&line)?);
    }

    Ok(rucksacks)
}

fn rucksacks_value(rs: &Vec<Rucksack>) -> u32 {
    rs.iter().map(|r| r.common().iter().map(eval_letter).sum::<u32>()).sum()
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let rucksacks = read_rucksacks(&file).expect("Cannot read rucksacks");

    match args.puzzle {
        Puzzle::P1 => println!("{}", rucksacks_value(&rucksacks)),
        Puzzle::P2 => todo!(),
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

    #[test]
    fn p1_total() {
        let rucksacks = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].iter().map(|s| build(s)).collect();

        assert_eq!(rucksacks_value(&rucksacks), 157);
    }
}
