use aoc::args::Puzzle;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Result as IOResult;
use std::str::FromStr;

struct Signal(Vec<char>);

impl FromStr for Signal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signal { 0: s.chars().collect() })
    }
}

impl Signal {
    fn start(&self, ws: usize) -> Option<usize> {
        for (index, chunk) in self.0.windows(ws).enumerate() {
            let set: HashSet<&char> = HashSet::from_iter(chunk);
            if set.len() == ws {
                return Some(ws + index);
            }
        }

        None
    }
}


fn read_signal(file: &File) -> Result<Signal, &'static str> {
    let mut lines = BufReader::new(file).lines();

    let line = aoc::io::read_line(&mut lines).ok_or("No line")?;
    Ok(line.parse()?)
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let signal = read_signal(&file).expect("Cannot read signal line");

    match args.puzzle {
        Puzzle::P1 => println!("{}", signal.start(4).expect("No signal start found")),
        Puzzle::P2 => println!("{}", signal.start(14).expect("No message start found")),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sig(s: &str) -> Signal {
        Signal::from_str(s).expect("Cannot parse signal")
    }

    #[test]
    fn p1_examples() {
        assert_eq!(sig("bvwbjplbgvbhsrlpgdmjqwftvncz").start(4), Some(5));
        assert_eq!(sig("nppdvjthqldpwncqszvftbrmjlhg").start(4), Some(6));
        assert_eq!(sig("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start(4), Some(10));
        assert_eq!(sig("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start(4), Some(11));
    }
}
