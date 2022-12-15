#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

pub struct DirectionIterator {
    view: &'static [Direction],
}

impl DirectionIterator {
    const ORDER: [Direction; 4] = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    pub fn new() -> Self {
        Self { view: &Self::ORDER }
    }
}

impl Default for DirectionIterator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for DirectionIterator {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.view.is_empty() {
            return None;
        }

        let rv = self.view.first().copied();
        self.view = &self.view[1..];
        rv
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub type Vector = Point;

impl Point where {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn shift(&self, v: &Vector) -> Self {
        Self::new(
            self.x + v.x,
            self.y + v.y,
        )
    }

    pub fn distance_from(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
}

impl PartialEq<(isize, isize)> for Point {
    fn eq(&self, other: &(isize, isize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl From<&Direction> for Vector {
    fn from(d: &Direction) -> Vector {
        match d {
            Direction::North => Vector::new(0, 1),
            Direction::South => Vector::new(0, -1),
            Direction::East => Vector::new(1, 0),
            Direction::West => Vector::new(-1, 0),
        }
    }
}

