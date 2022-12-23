use aoc::args::Puzzle;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    NoOp,
}

struct Program(Vec<Instruction>);

impl Program {
    fn new() -> Self {
        Program(Vec::new())
    }

    fn exec(&self) -> SignalIterator<std::slice::Iter<Instruction>> {
        SignalIterator::new(self.0.iter())
    }

    fn draw(&self) {
        for (cycle, x) in self.exec().enumerate() {
            let pos = (cycle % 40) as isize;
            print!("{}", if x - 1 <= pos && pos <= x + 1 { '█' } else { ' ' });

            if cycle % 40 == 39 {
                println!();
            }
        }
    }
}

struct SignalIterator<'a, InstrIt: Iterator<Item = &'a Instruction>> {
    x: isize,
    clock: isize,
    running: bool,
    program: std::iter::Peekable<InstrIt>,
}

impl<'a, ProgIt> SignalIterator<'a, ProgIt>
        where ProgIt: Iterator<Item = &'a Instruction> {
    fn new(program: ProgIt) -> Self {
        Self { x: 1, clock: 0, running: true, program: program.peekable() }
    }

    fn advance(&mut self) {
        self.program.next();
        self.clock = 0;
    }

    fn measure(self, cycles: &[isize]) -> Vec<isize> {
        let mut cycles = Vec::from(cycles);
        cycles.sort();

        let mut enm = self.enumerate();
        let mut res = Vec::with_capacity(cycles.len());

        for n in cycles {
            // Signals cycles are indexed from 1, not 0.
            for (i, value) in enm.by_ref() {
                assert!(i <= n as usize);
                if i + 1 == n as usize {
                    res.push((i + 1) as isize * value);
                    break;
                }
            }
        }

        res
    }
}

impl<'a, ProgIt: Iterator> Iterator for SignalIterator<'a, ProgIt>
        where ProgIt: Iterator<Item = &'a Instruction> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.x;
        match self.program.peek() {
            None => {
                self.running = false;
                return None;
            },
            Some(Instruction::NoOp) => self.advance(),
            Some(Instruction::AddX(_)) if self.clock < 1 => {
                self.clock += 1;
            }
            Some(Instruction::AddX(n)) => {
                self.advance();
                self.x += n;
            }
        };

        Some(ret)
    }
}

fn read_program(file: &File) -> Result<Program, String> {
    let mut line_reader = BufReader::new(file).lines();

    let mut program: Program = Program::new();
    while let Some(line) = aoc::io::read_line(&mut line_reader) {
        if line == "noop" {
            program.0.push(Instruction::NoOp);
        } else if let Some((left, right)) = line.split_once(' ') {
            if left != "addx" {
                return Err(String::from("Bad instruction: ") + &line);
            }

            let num = right.parse().map_err(|_| String::from("Invalid number: ") + right)?;
            program.0.push(Instruction::AddX(num));
        } else {
            return Err(String::from("Invalid line: ") + &line);
        }
    }

    Ok(program)
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let program = read_program(&file).expect("Cannot read program");

    match args.puzzle {
        Puzzle::P1 => {
            let iter = program.exec();
            let points = (20..=220).step_by(40).collect::<Vec<isize>>();
            println!("{}", iter.measure(&points).iter().sum::<isize>());
        },

        Puzzle::P2 => {
            program.draw();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data1() -> Program {
        Program(vec![
            Instruction::NoOp,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ])
    }

    fn data2() -> Program {
        Program(vec![
            Instruction::AddX(15),
            Instruction::AddX(-11),
            Instruction::AddX(6),
            Instruction::AddX(-3),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(-8),
            Instruction::AddX(13),
            Instruction::AddX(4),
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(-35),
            Instruction::AddX(1),
            Instruction::AddX(24),
            Instruction::AddX(-19),
            Instruction::AddX(1),
            Instruction::AddX(16),
            Instruction::AddX(-11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(21),
            Instruction::AddX(-15),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-3),
            Instruction::AddX(9),
            Instruction::AddX(1),
            Instruction::AddX(-3),
            Instruction::AddX(8),
            Instruction::AddX(1),
            Instruction::AddX(5),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-36),
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::AddX(7),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(2),
            Instruction::AddX(6),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(7),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(-13),
            Instruction::AddX(13),
            Instruction::AddX(7),
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::AddX(-33),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(2),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(8),
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::AddX(2),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(17),
            Instruction::AddX(-9),
            Instruction::AddX(1),
            Instruction::AddX(1),
            Instruction::AddX(-3),
            Instruction::AddX(11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-13),
            Instruction::AddX(-19),
            Instruction::AddX(1),
            Instruction::AddX(3),
            Instruction::AddX(26),
            Instruction::AddX(-30),
            Instruction::AddX(12),
            Instruction::AddX(-1),
            Instruction::AddX(3),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-9),
            Instruction::AddX(18),
            Instruction::AddX(1),
            Instruction::AddX(2),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(9),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::AddX(2),
            Instruction::AddX(-37),
            Instruction::AddX(1),
            Instruction::AddX(3),
            Instruction::NoOp,
            Instruction::AddX(15),
            Instruction::AddX(-21),
            Instruction::AddX(22),
            Instruction::AddX(-6),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(2),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(-10),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(20),
            Instruction::AddX(1),
            Instruction::AddX(2),
            Instruction::AddX(2),
            Instruction::AddX(-6),
            Instruction::AddX(-11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
        ])
    }

    #[test]
    fn example1() {
        let data1 = data1();
        let mut iter = SignalIterator::new(data1.0.iter());
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn example2() {
        let data1 = data2();
        let iter = data1.exec();
        assert_eq!(iter.measure(&(20..=220).step_by(40).collect::<Vec<isize>>()),
            vec![420, 1140, 1800, 2940, 2880, 3960]);
    }

    #[test]
    fn example2_sum() {
        let data2 = data2();
        let iter = data2.exec();
        assert_eq!(iter.measure(&(20..=220).step_by(40).collect::<Vec<isize>>()).iter().sum::<isize>(),
            13140);
    }
}
