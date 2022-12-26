mod path_segment {
    use aoc::euclid::Point;
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct PathSegment(pub Vec<Point>);

    impl FromStr for PathSegment {
        type Err = String;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            parser::path_segment(input)
        }
    }

    impl PathSegment {
        pub fn size(&self) -> Point {
            use std::cmp::max;

            self.0.iter().fold(Point::new(0, 0),
                |b, p| Point::new(max(b.x, p.x), max(b.y, p.y))
            )
        }
    }

    mod parser {
        use super::{PathSegment, Point};

        use nom::{
            bytes::complete::tag,
            character::complete::{char, digit1},
            combinator::map_res,
            multi::separated_list1,
            sequence::separated_pair,
            Finish, IResult,
        };

        fn coordinate(input: &str) -> IResult<&str, isize> {
            map_res(digit1, |n: &str| n.parse::<isize>())(input)
        }

        fn point(input: &str) -> IResult<&str, Point> {
            let (input, (x, y)) = separated_pair(
                coordinate, char(','), coordinate
            )(input)?;

            Ok((input, Point::new(x, y)))
        }

        fn path(input: &str) -> IResult<&str, Vec<Point>> {
            separated_list1(tag(" -> "), point)(input)
        }

        pub fn path_segment(input: &str) -> Result<PathSegment, String> {
            match path(input).finish() {
                Ok((rest, list)) if rest.is_empty() => Ok(PathSegment(list)),
                Ok((rest, _)) => Err(String::from("Junk trailing chars: ") + rest),
                Err(err) => Err(err.to_string()),
            }
        }
    }
}

mod scan {
    use super::path_segment::PathSegment;
    use aoc::euclid::Point;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub struct Scan(pub Vec<PathSegment>);

    impl Scan {
        pub fn new_from_file(file: &File) -> Result<Scan, String> {
            let mut paths = Vec::<PathSegment>::new();
            let mut lines = BufReader::new(file).lines();

            while let Some(line) = aoc::io::read_line(&mut lines) {
                paths.push(line.parse::<PathSegment>()?);
            }

            Ok(Scan(paths))
        }

        pub fn size(&self) -> Point {
            use std::cmp::max;

            self.0.iter().map(PathSegment::size).fold(
                Point::default(),
                |b, p| Point::new(max(b.x, p.x), max(b.y, p.y))
            )
        }

        pub fn add_floor(&mut self, source: &Point) {
            let floor_y = self.size().y + 2;
            self.0.push(PathSegment(vec![
                Point::new(source.x - (floor_y + 10), floor_y),
                Point::new(source.x + (floor_y + 10), floor_y),
            ]));
        }
    }
}

mod map {
    use super::{path_segment::PathSegment, scan::Scan};
    use aoc::euclid::{Point, Vector};
    use aoc::matrix::Matrix;
    use std::fs::File;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub enum Tile {
        Empty,
        Sand,
        Rock,
    }

    impl Default for Tile {
        fn default() -> Self {
            Self::Empty
        }
    }

    pub struct Map(Matrix<Tile>);

    impl Map {
        fn fill_segment(map: &mut Matrix<Tile>, segment: &PathSegment) {
            for win in segment.0.windows(2) {
                assert!(win[0].x == win[1].x || win[0].y == win[1].y);
                let dir = win[0].direction(&win[1]);

                let mut cursor = win[0];
                while cursor != win[1] {
                    map[cursor] = Tile::Rock;
                    cursor = cursor.shift(&dir);
                }

                map[cursor] = Tile::Rock;
            }
        }

        pub fn new_from_scan(scan: &Scan) -> Self {
            let size = scan.size();
            let mut map = Matrix::new(size.x as usize + 1, size.y as usize + 1);

            for segment in &scan.0 {
                Self::fill_segment(&mut map, segment);
            }

            Self(map)
        }

        #[allow(dead_code)]
        pub fn new_from_file(file: &File) -> Result<Map, String> {
            Ok(Self::new_from_scan(&super::scan::Scan::new_from_file(file)?))
        }

        #[allow(dead_code)]
        pub fn draw(&self) {
            println!("Map:");
            for row in &self.0.data {
                for cell in row {
                    print!("{}", match cell {
                        Tile::Empty => '.',
                        Tile::Rock => '#',
                        Tile::Sand => '%',
                    })
                }

                println!();
            }
        }

        pub fn drop_sand(&mut self, from: &Point) -> Result<Point, Point> {
            let mut current = *from;

            if self.0[current] != Tile::Empty {
                return Err(current);
            }

            'loc: loop {
                let locations = [
                    current.shift(&Vector::new(0, 1)),
                    current.shift(&Vector::new(-1, 1)),
                    current.shift(&Vector::new(1, 1)),
                ];

                for next in locations {
                    if !self.0.contains(&next) {
                        return Err(next);
                    }

                    if self.0[next] == Tile::Empty {
                        current = next;
                        continue 'loc;
                    }
                }

                self.0[current] = Tile::Sand;
                break;
            }

            Ok(current)
        }

        pub fn fill(&mut self, from: &Point) -> usize {
            std::iter::repeat_with(|| self.drop_sand(from))
                .take_while(|r| r.is_ok())
                .count()
        }
    }
}

use aoc::args::Puzzle;
use aoc::euclid::Point;
use map::Map;
use scan::Scan;
use std::fs::File;

fn main() -> Result<(), String> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name).expect("Cannot open file");

    let source = Point::new(500, 0);
    let mut scan = Scan::new_from_file(&file)?;

    if args.puzzle == Puzzle::P2 {
        scan.add_floor(&source);
    }

    let mut map = Map::new_from_scan(&scan);
    println!("{}", map.fill(&source));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc::euclid::Point;

    fn example_scan() -> scan::Scan {
        scan::Scan(vec![
            path_segment::PathSegment(vec![
                Point::new(498, 4),
                Point::new(498, 6),
                Point::new(496, 6),
            ]),
            path_segment::PathSegment(vec![
                Point::new(503, 4),
                Point::new(502, 4),
                Point::new(502, 9),
                Point::new(494, 9),
            ]),
        ])
    }

    #[test]
    fn example1() {
        let mut map = map::Map::new_from_scan(&example_scan());
        assert_eq!(map.fill(&Point::new(500, 0)), 24);
    }

    #[test]
    fn example2() {
        let source = Point::new(500, 0);
        let mut scan = example_scan();
        scan.add_floor(&source);

        let mut map = map::Map::new_from_scan(&scan);
        assert_eq!(map.fill(&source), 93);
    }
}
