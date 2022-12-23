use aoc::args::Puzzle;
use aoc::euclid::{CoordGenerator, Direction, DirectionIterator, Point, Vector};
use aoc::matrix::{Matrix};
use std::cmp::max;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};

struct Map(Matrix<u32>);
struct BitLayer(Matrix<bool>);

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self(Matrix::new(width, height))
    }

    fn layer(&self, dir: &Direction) -> BitLayer {
        let mut layer = BitLayer::new(self.0.width, self.0.height);
        let mut last = Point::new(-1, -1);
        let mut max_elevation = 0;

        for coord in CoordGenerator::new(dir, self.0.width, self.0.height) {
            /* If both coords change, we have a different set of points. */
            if coord.x != last.x && coord.y != last.y {
                layer.0[coord] = true;
                max_elevation = self.0[coord];
            } else {
                layer.0[coord] = self.0[coord] > max_elevation;
                max_elevation = max(max_elevation, self.0[coord])
            }

            last = coord;
        }

        layer
    }

    pub fn elevated_points(&self) -> usize {
        DirectionIterator::new()
            .map(|dir| self.layer(&dir))
            .fold(BitLayer::new(self.0.width, self.0.height),
                    |acc, el| BitLayer::or(&acc, &el))
            .bits(true)
    }

    fn is_border(&self, coord: &Point) -> bool {
        coord.x == 0 || coord.y == 0
            || coord.x + 1 == self.0.width as isize
            || coord.y + 1 == self.0.height as isize
    }

    fn scenic_ray(&self, coord: &Point, v: &Vector) -> usize {
        if self.is_border(coord) {
            return 0;
        }

        let elev = self.0[*coord];
        let mut cursor: Point = *coord;

        let mut count: usize = 0;
        while !self.is_border(&cursor) && (cursor == *coord || self.0[cursor] < elev) {
            count += 1;
            cursor = cursor.shift(v);
        }

        count
    }

    fn scenic_scores(&self) -> usize {
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();

        for coord in CoordGenerator::new(
                &Direction::North, self.0.width, self.0.height) {
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
    pub fn new(width: usize, height: usize) -> Self {
        Self(Matrix::new(width, height))
    }

    pub fn or(a: &Self, b: &Self) -> Self {
        Self(Matrix::fold(&a.0, &b.0, |p, q| *p || *q))
    }

    pub fn bits(&self, b: bool) -> usize {
        let mut counter: usize = 0;
        for row in &self.0.data {
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
            map.0[coord] = c.to_digit(10).expect("BUG: Invalid digit found");
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
        Map(Matrix {
            width: 5,
            height: 5,
            data: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        })
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
