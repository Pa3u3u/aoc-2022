use aoc::args::Puzzle;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Lines};

type Pack = Vec<u32>;
type Expedition = Vec<Pack>;

fn read_pack<T>(lines: &mut Lines<T>) -> Option<Pack>
        where T: BufRead {
    let mut it = lines
        .map(|val| val.unwrap())
        .take_while(|line| !line.is_empty())
        .peekable();

    it.peek()?;

    Some(
        it.map(|line| line.parse().expect("Cannot parse value"))
            .collect(),
    )
}

fn read_expedition(file: &File) -> Expedition {
    let mut lines = BufReader::new(file).lines();
    let mut packs = Expedition::new();

    while let Some(pack) = read_pack(&mut lines) {
        packs.push(pack);
    }

    packs
}

fn sum_calories(n: usize, expedition: &Expedition) -> u32 {
    assert!(expedition.len() >= n);

    let mut sorted_expedition: Vec<u32> = expedition.iter()
        .map(|v| v.iter().sum())
        .collect();

    sorted_expedition.sort_by(|a, b| b.cmp(a));

    sorted_expedition.iter().take(n).sum()
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let expedition = read_expedition(&file);

    match args.puzzle {
        Puzzle::P1 => println!("{}", sum_calories(1, &expedition)),
        Puzzle::P2 => println!("{}", sum_calories(3, &expedition)),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example1() -> Expedition {
        vec![
            vec![1_000, 2_000, 3_000],
            vec![4_000],
            vec![5_000, 6_000],
            vec![7_000, 8_000, 9_000],
            vec![10_000],
        ]
    }

    #[test]
    fn p1_example1() {
        assert_eq!(sum_calories(1, &example1()), 24000);
    }

    #[test]
    fn p2_example1() {
        assert_eq!(sum_calories(3, &example1()), 45000);
    }
}
