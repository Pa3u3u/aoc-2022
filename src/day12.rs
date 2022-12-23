mod map {
    use aoc::euclid::{Point, Vector};
    use aoc::matrix::{Matrix};
    use std::collections::{BinaryHeap, BTreeSet};
    use std::cmp::Reverse;

    pub struct Map {
        start: Point,
        finish: Point,
        map: Matrix<isize>,
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct SearchItem {
        dist: usize,
        point: Point,
        height: isize,
    }

    impl SearchItem {
        fn new(dist: usize, point: Point, height: isize) -> Self {
            Self { dist, point, height }
        }
    }

    trait SearchMode {
        fn finish(&self, item: &SearchItem) -> bool;
        fn accept(&self, item: &SearchItem, next: &SearchItem) -> bool;
    }

    struct ClimbUp(Point);

    impl SearchMode for ClimbUp {
        fn finish(&self, item: &SearchItem) -> bool {
            item.point == self.0
        }

        fn accept(&self, item: &SearchItem, next: &SearchItem) -> bool {
            item.height + 1 >= next.height
        }
    }

    struct ClimbDown(isize);

    impl SearchMode for ClimbDown {
        fn finish(&self, item: &SearchItem) -> bool {
            item.height == self.0
        }

        fn accept(&self, item: &SearchItem, next: &SearchItem) -> bool {
            next.height + 1 >= item.height
        }
    }

    impl Map {
        pub fn new(start: Point, finish: Point, map: Matrix<isize>) -> Self {
            assert!(map.contains(&start));
            assert!(map.contains(&finish));

            Self { start, finish, map }
        }

        fn dijkstra<M: SearchMode>(&self, start: Point, mode: &M) -> Option<usize> {
            assert!(self.map.contains(&start));

            let mut heap = BinaryHeap::new();
            let mut marked = BTreeSet::new();

            heap.push(Reverse(SearchItem::new(0, start, self.map[start])));
            marked.insert(start);

            while let Some(Reverse(item)) = heap.pop() {
                if mode.finish(&item) {
                    return Some(item.dist);
                }

                for vec in aoc::euclid::DirectionIterator::new().map(Vector::from) {
                    let p = item.point.shift(&vec);

                    if !self.map.contains(&p) || marked.contains(&p) {
                        continue;
                    }

                    let next = SearchItem::new(item.dist + 1, p, self.map[p]);

                    if mode.accept(&item, &next) {
                        heap.push(Reverse(next));
                        marked.insert(p);
                    }
                }
            }

            None
        }

        pub fn shortest_path(&self) -> Option<usize> {
            self.dijkstra(self.start, &ClimbUp(self.finish))
        }

        pub fn scenic_path(&self) -> Option<usize> {
            self.dijkstra(self.finish, &ClimbDown(0))
        }
    }

    use std::convert::TryFrom;
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    impl TryFrom<File> for Map {
        type Error = String;

        fn try_from(file: std::fs::File) -> Result<Self, Self::Error> {
            fn char_to_val(c: char) -> isize {
                assert!(c.is_lowercase());
                (c as isize) - ('a' as isize)
            }

            let mut line_reader = BufReader::new(file).lines();
            let mut lines: Vec<String> = Vec::new();

            while let Some(line) = aoc::io::read_line(&mut line_reader) {
                if line.chars().any(|c| !c.is_ascii_alphabetic()) {
                    return Err(String::from("Invalid line: ") + line.as_str());
                }

                lines.push(line);
            }

            if lines.is_empty() {
                return Err(String::from("Empty input"));
            }

            if lines.iter().any(|l| l.len() != lines[0].len()) {
                return Err(String::from("Lines of input do not have the same size"));
            }

            let mut map = Matrix::new(lines[0].len(), lines.len());
            let mut coord = Point::new(0, 0);
            let mut start: Option<Point> = Option::None;
            let mut end: Option<Point> = Option::None;

            for row in &lines {
                for c in row.chars() {
                    map[coord] = match c {
                        'S' if start.is_none() => {
                            start = Some(coord);
                            char_to_val('a')
                        }
                        'E' if end.is_none() => {
                            end = Some(coord);
                            char_to_val('z')
                        }
                        chr if chr.is_lowercase() => {
                            char_to_val(chr)
                        }
                        c => {
                            return Err(
                                format!("[{}, {}]: Unexpected {}", coord.x, coord.y, c));
                        }
                    };

                    coord.x += 1;
                }

                coord = Point::new(0, coord.y + 1);
            }

            Ok(Map::new(
                    start.ok_or_else(|| String::from("No start found"))?,
                    end.ok_or_else(|| String::from("No end found"))?,
                    map))
        }
    }
}

use std::fs::File;
use aoc::args::Puzzle;

fn main() -> Result<(), String> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name).expect("Cannot open file");

    let map = map::Map::try_from(file)?;

    match args.puzzle {
        Puzzle::P1 => {
            println!("{}", map.shortest_path().unwrap());
        }
        Puzzle::P2 => {
            println!("{}", map.scenic_path().unwrap());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc::euclid::Point;
    use aoc::matrix::Matrix;
    use super::*;

    fn example1() -> map::Map {
        map::Map::new(Point::new(0, 0), Point::new(5, 2), Matrix {
            width: 8,
            height: 5,
            data: vec![
                vec![0, 0, 1, 16, 15, 14, 13, 12],
                vec![0, 1, 2, 17, 24, 23, 23, 11],
                vec![0, 2, 2, 18, 25, 25, 23, 10],
                vec![0, 2, 2, 19, 20, 21, 22, 9],
                vec![0, 1, 3, 4, 5, 6, 7, 8],
            ],
        })
    }

    #[test]
    fn example_distance() {
        let map = example1();
        assert_eq!(map.shortest_path(), Some(31));
    }

    #[test]
    fn example_scenic() {
        let map = example1();
        assert_eq!(map.scenic_path(), Some(29));
    }
}
