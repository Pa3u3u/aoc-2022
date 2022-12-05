use aoc::args::Puzzle;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;
use std::cell::RefCell;

type Stack = Vec<char>;

#[derive(Clone)]
struct Ship {
    crates: Vec<RefCell<Stack>>,
}

impl Ship {
    pub fn top(&self) -> Result<Vec<char>, &'static str> {
        let chars = self.crates.iter()
            .map(|vec| vec.borrow().last().ok_or("Cargo stack is empty").map(|c| *c));

        chars.collect()
    }

    pub fn top_str(&self) -> Result<String, &'static str> {
        Ok(String::from_iter(self.top()?.iter()))
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    src: usize,
    dst: usize,
    count: u32,
}

lazy_static! {
    static ref INSTR_RE: Regex =
        Regex::new(r"move ([1-9]\d*) from ([1-9]\d*) to ([1-9]\d*)").unwrap();
}

impl Instruction {
    fn new(src: usize, dst: usize, count: u32) -> Instruction {
        Instruction { src, dst, count }
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = INSTR_RE.captures(s)
            .ok_or("Line does not match expected pattern")?;

        Ok(Instruction::new(
            cap.get(2).expect("No source match")
                    .as_str().parse::<usize>().unwrap() - 1,
            cap.get(3).expect("No target match")
                    .as_str().parse::<usize>().unwrap() - 1,
            cap.get(1).expect("No count match")
                    .as_str().parse::<u32>().unwrap(),
        ))
    }
}

#[derive(Clone)]
struct RearrProc {
    ship: Ship,
    plan: VecDeque<Instruction>,
}

trait CrateMover {
    fn exec(stacks: &mut Vec<RefCell<Stack>>, instr: &Instruction)
        -> Result<(), &'static str>;
}

struct CrateMover9000;

impl CrateMover for CrateMover9000 {
    fn exec(stacks: &mut Vec<RefCell<Stack>>, instr: &Instruction)
            -> Result<(), &'static str> {
        if instr.src == instr.dst || instr.count == 0 {
            return Ok(());
        }

        for _ in 0 .. instr.count {
            let cargo = stacks[instr.src].borrow_mut().pop()
                            .ok_or("Source stack is empty")?;
            stacks[instr.dst].borrow_mut().push(cargo);
        }

        Ok(())
    }
}

struct CrateMover9001;

impl CrateMover for CrateMover9001 {
    fn exec(stacks: &mut Vec<RefCell<Stack>>, instr: &Instruction)
            -> Result<(), &'static str> {
        if instr.src == instr.dst || instr.count == 0 {
            return Ok(());
        }

        // Condition ‹instr.src ≠ instr.dst› is guaranteed above.
        let mut src = stacks[instr.src].borrow_mut();
        let mut dst = stacks[instr.dst].borrow_mut();

        let skip = src.len() - instr.count as usize;
        let it = src.iter().skip(skip);
        dst.extend(it);
        src.truncate(skip);

        Ok(())
    }
}

impl RearrProc {
    pub fn run<CM: CrateMover>(&mut self) -> Result<(), &'static str> {
        while let Some(instr) = self.plan.pop_front() {
            CM::exec(&mut self.ship.crates, &instr)?;
        }

        Ok(())
    }
}

enum ShipPart {
    End,
    Part(Vec<Option<char>>),
    Check(usize),
}

fn read_ship_part(line: &str) -> Result<ShipPart, &'static str> {
    // Empty line marks the end of ship parts.
    if line.is_empty() {
        return Ok(ShipPart::End);
    }

    // The length of a string with ⟦n⟧ crates is ⟦3n + (n - 1) = 4n - 1⟧,
    // thus we only need to test if ⟦n + 1⟧ is divisible by four.
    if (line.len() + 1) % 4 != 0 {
        return Err("Invalid line length");
    }

    let crates_count = (line.len() + 1) / 4;
    let mut crates: Vec<Option<char>> = Vec::new();

    for i in 0 .. crates_count {
        let c = line.chars().nth(4 * i + 1)
                    .expect("BUG: Too few characters on line");
        crates.push(if c == ' ' { None } else { Some(c) });
    }

    if crates.iter().all(|w| w.map_or(false, |c| c.is_ascii_digit())) {
        // Check that number line up.
        for (i, crte) in crates.iter().enumerate() {
            let index = crte.ok_or("BUG: Too few indices in check row")?
                    .to_digit(10).ok_or("BUG: Not a digit")?;

            if index as usize != i + 1 {
                return Err("Mismatched row numbers");
            }
        }

        return Ok(ShipPart::Check(crates_count));
    }

    Ok(ShipPart::Part(crates))
}

fn read_ship<T>(lines: &mut Lines<T>) -> Result<Ship, &'static str>
        where T: BufRead {
    let mut parts: Vec<VecDeque<char>> = Vec::new();

    while let Some(line) = aoc::io::read_line(lines) {
        match read_ship_part(&line)? {
            ShipPart::End => break,
            ShipPart::Part(part) => {
                if parts.len() < part.len() {
                    parts.resize(part.len(), VecDeque::new());
                }

                for (i, stack) in parts.iter_mut().enumerate() {
                    if let Some(c) = part.get(i).unwrap() {
                        stack.push_front(*c);
                    }
                }
            }
            ShipPart::Check(n) => {
                if parts.is_empty() {
                    return Err("Empty ship");
                }

                if parts.len() != n {
                    return Err("Invalid ship size");
                }
            }
        }
    }

    let crates = parts.iter().map(|p| RefCell::new(p.iter().copied().collect())).collect();
    Ok(Ship { crates })
}

fn read_instructions<T>(lines: &mut Lines<T>) -> Result<Vec<Instruction>, &'static str>
        where T: BufRead {
    let mut instr: Vec<Instruction> = Vec::new();

    while let Some(line) = aoc::io::read_line(lines) {
        instr.push(line.parse()?);
    }

    Ok(instr)
}

fn read_procedure(file: &File) -> Result<RearrProc, &'static str> {
    let mut lines = BufReader::new(file).lines();
    let ship = read_ship(&mut lines)?;
    let plan = read_instructions(&mut lines)?.into();

    Ok(RearrProc { ship, plan })
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let mut procedure = read_procedure(&file).expect("Cannot read procedure");

    match args.puzzle {
        Puzzle::P1 => procedure.run::<CrateMover9000>(),
        Puzzle::P2 => procedure.run::<CrateMover9001>(),
    }.expect("Failed to run the procedure");

    println!("{}", procedure.ship.top_str().expect("Cannot get top string"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack_from_str(s: &str) -> RefCell<Stack> {
        s.chars().collect::<Vec<char>>().into()
    }

    fn example_ship() -> Ship {
        Ship {
            crates: vec![
                stack_from_str("ZN"),
                stack_from_str("MCD"),
                stack_from_str("P"),
            ],
        }
    }

    #[test]
    fn p1_top_no_change() {
        let example_ship = example_ship();

        assert_eq!(example_ship.top_str().expect("Cannot get top row"), "NDP");
    }

    #[test]
    fn p1_single_instr() {
        let mut rp = RearrProc {
            ship: example_ship(),
            plan: [Instruction::new(1, 0, 1)].into(),
        };

        rp.run::<CrateMover9000>().expect("Plan failed");
        assert_eq!(rp.ship.top_str().expect("Cannot get top row"), "DCP");
    }

    #[test]
    fn p1_complete_plan() {
        let mut rp = RearrProc {
            ship: example_ship(),
            plan: [
                Instruction::new(1, 0, 1),
                Instruction::new(0, 2, 3),
                Instruction::new(1, 0, 2),
                Instruction::new(0, 1, 1),
            ].into(),
        };

        rp.run::<CrateMover9000>().expect("Plan failed");
        assert_eq!(rp.ship.top_str().expect("Cannot get top row"), "CMZ");
    }

    #[test]
    fn p2_complete_plan() {
        let mut rp = RearrProc {
            ship: example_ship(),
            plan: [
                Instruction::new(1, 0, 1),
                Instruction::new(0, 2, 3),
                Instruction::new(1, 0, 2),
                Instruction::new(0, 1, 1),
            ].into(),
        };

        rp.run::<CrateMover9001>().expect("Plan failed");
        assert_eq!(rp.ship.top_str().expect("Cannot get top row"), "MCD");
    }
}
