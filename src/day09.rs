use aoc::args::Puzzle;
use aoc::euclid::{Direction, Point, Vector};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Rope {
    head: Point,
    tail: Point,
}

fn points_are_near(a: &Point, b: &Point) -> bool {
    a.distance_from(b) < 2.0
}

impl Rope {
    fn new(init: Point) -> Self {
        Self { head: init, tail: init }
    }

    fn walk_tail(&self, nh: &Point) -> Point {
        if points_are_near(nh, &self.tail) {
            return self.tail;
        }

        self.head
    }

    /* 'move' is a keyword in Rust /o\ */
    fn walk(&self, direction: Direction) -> Self {
        let v = Vector::from(&direction);

        let new_head = self.head.shift(&v);
        let new_tail = self.walk_tail(&new_head);

        Self { head: new_head, tail: new_tail }
    }
}

struct Motion {
    dir: Direction,
    count: usize,
}

impl Motion {
    fn new(dir: Direction, count: usize) -> Self {
        Self { dir, count }
    }

    fn _parse_dir(s: &str) -> Option<Direction> {
        match s {
            "R" => Some(Direction::East),
            "U" => Some(Direction::North),
            "L" => Some(Direction::West),
            "D" => Some(Direction::South),
            _ => None,
        }
    }
}

impl FromStr for Motion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, count) = s.split_once(' ')
                            .ok_or_else(|| String::from("Invalid delimiter"))?;

        let dir = Self::_parse_dir(dir)
                        .ok_or_else(|| String::from("Invalid direction: ") + dir)?;
        let count = count.parse::<usize>()
                        .map_err(|x| x.to_string())?;

        Ok(Motion::new(dir, count))
    }
}

struct Simulation(Vec<Motion>);

impl Simulation {
    fn new() -> Simulation {
        Simulation(Vec::new())
    }

    fn run(&self, start: &Rope, observer: &mut dyn Observer) {
        observer.observe(start);
        let mut current: Rope = *start;

        for motion in &self.0 {
            for _ in 0 .. motion.count {
                current = current.walk(motion.dir);
                observer.observe(&current);
            }
        }
    }
}

trait Observer {
    fn observe(&mut self, rope: &Rope);
}

#[derive(Default)]
struct TailObserver {
    map: BTreeMap<Point, usize>,
}

impl TailObserver {
    fn new() -> Self {
        Self::default()
    }

    fn result(&self) -> usize {
        self.map.len()
    }
}

impl Observer for TailObserver {
    fn observe(&mut self, rope: &Rope) {
        match self.map.entry(rope.tail) {
            Entry::Occupied(mut occupied) => {
                *occupied.get_mut() += 1;
            }
            Entry::Vacant(vacant) => {
                vacant.insert(1);
            }
        }
    }
}

fn read_simulation(file: &File) -> Result<Simulation, String> {
    let mut line_reader = BufReader::new(file).lines();

    let mut sim: Simulation = Simulation::new();
    while let Some(line) = aoc::io::read_line(&mut line_reader) {
        sim.0.push(line.parse()?);
    }

    Ok(sim)
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let simulation = read_simulation(&file).expect("Cannot parse simulation");

    match args.puzzle {
        Puzzle::P1 => {
            let mut observer = TailObserver::new();
            simulation.run(&Rope::new(Point::new(0, 0)), &mut observer);
            println!("{}", observer.result());
        }

        Puzzle::P2 => todo!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rope(h: (isize, isize), t: (isize, isize)) -> Rope {
        Rope { head: Point::new(h.0, h.1), tail: Point::new(t.0, t.1) }
    }

    #[test]
    fn moving() {
        assert_eq!(rope((0, 0), (0, 0)).walk(Direction::North),
                   rope((0, 1), (0, 0)));
        assert_eq!(rope((1, 1), (0, 0)).walk(Direction::North),
                   rope((1, 2), (1, 1)));
        assert_eq!(rope((1, 0), (0, 0)).walk(Direction::East),
                   rope((2, 0), (1, 0)));
        assert_eq!(rope((2, 2), (2, 2)).walk(Direction::West),
                   rope((1, 2), (2, 2)));
        assert_eq!(rope((3, 2), (2, 2)).walk(Direction::West),
                   rope((2, 2), (2, 2)));
    }

    fn example() -> Simulation {
        Simulation(vec![
            Motion::new(Direction::East, 4),
            Motion::new(Direction::North, 4),
            Motion::new(Direction::West, 3),
            Motion::new(Direction::South, 1),
            Motion::new(Direction::East, 4),
            Motion::new(Direction::South, 1),
            Motion::new(Direction::West, 5),
            Motion::new(Direction::East, 2),
        ])
    }

    #[test]
    fn example1() {
        let mut observer = TailObserver::new();
        let start = Rope::new(Point::new(0, 0));

        example().run(&start, &mut observer);

        assert_eq!(observer.result(), 13);
    }
}
