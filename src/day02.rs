use aoc::args::Puzzle;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Eq)]
enum MatchResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn cmp_round(left: &Shape, right: &Shape) -> MatchResult {
    match (left, right) {
        (x, y) if x == y => MatchResult::Draw,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Scissors, Shape::Paper)
        | (Shape::Paper, Shape::Rock) => MatchResult::Lose,
        _ => MatchResult::Win,
    }
}

type Round = (Shape, Shape);

fn eval_round(r: &Round) -> usize {
    (r.1 as usize) + (cmp_round(&r.0, &r.1) as usize)
}

type Strategy = Vec<Round>;

fn eval_strategy(s: &Strategy) -> usize {
    s.iter().map(eval_round).sum()
}

fn parse_shape(part: &str) -> Option<Shape> {
    match part {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn parse_result(part: &str) -> Option<MatchResult> {
    match part {
        "X" => Some(MatchResult::Lose),
        "Y" => Some(MatchResult::Draw),
        "Z" => Some(MatchResult::Win),
        _ => None,
    }
}

fn read_parts(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() != 2 {
        panic!("Invalid input");
    }

    Some((parts[0], parts[1]))
}

fn read_round_1(line: &str) -> Option<Round> {
    let (sl, sr) = read_parts(line)?;

    let left = parse_shape(sl);
    let right = parse_shape(sr);

    left.zip(right)
}

fn find_match(left: &Shape, expected: &MatchResult) -> Shape {
    for shape in [Shape::Rock, Shape::Paper, Shape::Scissors] {
        if cmp_round(left, &shape) == *expected {
            return shape;
        }
    }

    panic!("BUG: find_match(): Exhausted shape search space")
}

fn read_round_2(line: &str) -> Option<Round> {
    let (sl, sr) = read_parts(line)?;

    let left = parse_shape(sl)?;
    let expected = parse_result(sr)?;

    Some((left, find_match(&left, &expected)))
}

fn read_strategy(file: &File, reader: &dyn Fn(&str) -> Option<Round>) -> Strategy {
    let mut lines = BufReader::new(file).lines();
    let mut strategy = Strategy::new();

    while let Some(line) = aoc::io::read_line(&mut lines) {
        if let Some(round) = reader(&line) {
            strategy.push(round);
        }
    }

    strategy
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    println!("{}", eval_strategy(&read_strategy(&file,
        &match args.puzzle {
            Puzzle::P1 => read_round_1,
            Puzzle::P2 => read_round_2,
        }
    )));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example1() -> Strategy {
        vec![
            (Shape::Rock, Shape::Paper),
            (Shape::Paper, Shape::Rock),
            (Shape::Scissors, Shape::Scissors),
        ]
    }

    #[test]
    fn p1_example1() {
        assert_eq!(eval_strategy(&example1()), 15);
    }

    #[test]
    fn p1_comparisons() {
        assert_eq!(eval_round(&(Shape::Rock, Shape::Paper)), 8);
        assert_eq!(eval_round(&(Shape::Paper, Shape::Rock)), 1);
        assert_eq!(eval_round(&(Shape::Scissors, Shape::Scissors)), 6);
    }

    #[test]
    fn p2_matches() {
        assert_eq!(find_match(&Shape::Rock, &MatchResult::Draw), Shape::Rock);
        assert_eq!(find_match(&Shape::Paper, &MatchResult::Lose), Shape::Rock);
        assert_eq!(find_match(&Shape::Scissors, &MatchResult::Win), Shape::Rock);
    }

    #[test]
    fn p2_comparisons() {
        assert_eq!(eval_round(&(Shape::Rock, Shape::Rock)), 4);
        assert_eq!(eval_round(&(Shape::Paper, Shape::Rock)), 1);
        assert_eq!(eval_round(&(Shape::Scissors, Shape::Rock)), 7);
    }
}
