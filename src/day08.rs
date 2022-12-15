use aoc::args::Puzzle;
use aoc::euclid::{Direction, DirectionIterator, Point, Vector};
use std::cmp::max;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct CoordGenerator {
    dir: Direction,
    cursor: isize,
    limit: [isize; 2],
}

impl CoordGenerator {
    pub fn new(dir: &Direction, width: usize, height: usize) -> CoordGenerator {
        Self {
            dir: *dir,
            cursor: 0,
            limit: [width as isize, height as isize],
        }
    }

    fn _fx(&self, v: isize, i: usize) -> isize {
        v % self.limit[i]
    }

    fn _fy(&self, v: isize, i: usize) -> isize {
        v / self.limit[i]
    }

    fn _lim(&self) -> isize {
        self.limit[0] * self.limit[1]
    }
}

impl Iterator for CoordGenerator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self._lim() {
            return None;
        }

        let cursor = self.cursor;
        let rev_cursor = self._lim() - cursor - 1;
        self.cursor += 1;

        let (x, y) = match self.dir {
            Direction::North => (self._fy(rev_cursor, 1), self._fx(rev_cursor, 1)),
            Direction::West => (self._fx(rev_cursor, 0), self._fy(rev_cursor, 0)),
            Direction::South => (self._fy(cursor, 1), self._fx(cursor, 1)),
            Direction::East => (self._fx(cursor, 0), self._fy(cursor, 0)),
        };

        Some(Point::new(x, y))
    }
}

#[derive(Debug)]
struct Matrix<T: Default + Ord> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

type Map = Matrix<u32>;
type BitLayer = Matrix<bool>;

impl<T> Matrix<T>
        where T: Default + Ord {
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        let mut data: Vec<Vec<T>> = Vec::with_capacity(height);

        data.resize_with(height, || {
            let mut vi = Vec::with_capacity(width);
            vi.resize_with(width, Default::default);
            vi
        });

        Self { width, height, data }
    }

    pub fn fold<F: Fn(&T, &T) -> T>(a: &Self, b: &Self, f: F) -> Self {
        assert_eq!(a.width, b.width);
        assert_eq!(a.height, b.height);

        let mut result = Self::new(a.width, a.height);
        for xy in CoordGenerator::new(&Direction::East, a.width, a.height) {
            result[xy] = f(&a[xy], &b[xy]);
        }

        result
    }
}

impl<T: Default + Ord> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.data[index.y as usize][index.x as usize]
    }
}

impl<T: Default + Ord> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.data[index.y as usize][index.x as usize]
    }
}

impl Map {
    fn layer(&self, dir: &Direction) -> BitLayer {
        let mut layer = BitLayer::new(self.width, self.height);
        let mut last = Point::new(-1, -1);
        let mut max_elevation = 0;

        for coord in CoordGenerator::new(dir, self.width, self.height) {
            /* If both coords change, we have a different set of points. */
            if coord.x != last.x && coord.y != last.y {
                layer[coord] = true;
                max_elevation = self[coord];
            } else {
                layer[coord] = self[coord] > max_elevation;
                max_elevation = max(max_elevation, self[coord])
            }

            last = coord;
        }

        layer
    }

    pub fn elevated_points(&self) -> usize {
        DirectionIterator::new()
            .map(|dir| self.layer(&dir))
            .fold(BitLayer::new(self.width, self.height),
                    |acc, el| BitLayer::or(&acc, &el))
            .bits(true)
    }

    fn is_border(&self, coord: &Point) -> bool {
        coord.x == 0 || coord.y == 0
            || coord.x + 1 == self.width as isize
            || coord.y + 1 == self.height as isize
    }

    fn scenic_ray(&self, coord: &Point, v: &Vector) -> usize {
        if self.is_border(coord) {
            return 0;
        }

        let elev = self[*coord];
        let mut cursor: Point = *coord;

        let mut count: usize = 0;
        while !self.is_border(&cursor) && (cursor == *coord || self[cursor] < elev) {
            count += 1;
            cursor = cursor.shift(v);
        }

        count
    }

    fn scenic_scores(&self) -> usize {
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();

        for coord in CoordGenerator::new(&Direction::North, self.width, self.height) {
            heap.push(
                DirectionIterator::new()
                    .map(|d| self.scenic_ray(&coord, &Vector::from(&d)))
                    .product(),
            );

            if heap.len() > 1000 {
                heap.shrink_to(100);
            }
        }

        heap.pop().expect("No elements found")
    }
}

impl BitLayer {
    pub fn or(a: &Self, b: &Self) -> Self {
        Self::fold(a, b, |p, q| *p || *q)
    }

    pub fn bits(&self, b: bool) -> usize {
        let mut counter: usize = 0;
        for row in &self.data {
            for cell in row {
                if *cell == b {
                    counter += 1;
                }
            }
        }

        counter
    }
}

fn read_map(file: &File) -> Result<Map, String> {
    let mut line_reader = BufReader::new(file).lines();
    let mut lines: Vec<String> = Vec::new();

    while let Some(line) = aoc::io::read_line(&mut line_reader) {
        if line.chars().any(|c| !c.is_ascii_digit()) {
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

    let mut map = Map::new(lines[0].len(), lines.len());

    let mut coord = Point::new(0, 0);
    for row in &lines {
        for c in row.chars() {
            map[coord] = c.to_digit(10).expect("BUG: Invalid digit found");
            coord.x += 1;
        }

        coord = Point::new(0, coord.y + 1);
    }

    Ok(map)
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let map = read_map(&file).expect("Cannot read input");

    println!("{}", match args.puzzle {
        Puzzle::P1 => map.elevated_points(),
        Puzzle::P2 => map.scenic_scores(),
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pt(x: isize, y: isize) -> Option<Point> {
        Some(Point::new(x, y))
    }

    #[test]
    fn gen_east() {
        let mut g = CoordGenerator::new(&Direction::East, 3, 2);
        assert_eq!(g.next(), pt(0, 0));
        assert_eq!(g.next(), pt(1, 0));
        assert_eq!(g.next(), pt(2, 0));
        assert_eq!(g.next(), pt(0, 1));
        assert_eq!(g.next(), pt(1, 1));
        assert_eq!(g.next(), pt(2, 1));
    }

    #[test]
    fn gen_north() {
        let mut g = CoordGenerator::new(&Direction::North, 3, 2);
        assert_eq!(g.next(), pt(2, 1));
        assert_eq!(g.next(), pt(2, 0));
        assert_eq!(g.next(), pt(1, 1));
        assert_eq!(g.next(), pt(1, 0));
        assert_eq!(g.next(), pt(0, 1));
        assert_eq!(g.next(), pt(0, 0));
    }

    fn example_matrix() -> Map {
        Matrix {
            width: 5,
            height: 5,
            data: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        }
    }

    #[test]
    fn example1() {
        assert_eq!(example_matrix().elevated_points(), 21);
    }

    #[test]
    fn example2() {
        assert_eq!(example_matrix().scenic_scores(), 8);
    }
}
