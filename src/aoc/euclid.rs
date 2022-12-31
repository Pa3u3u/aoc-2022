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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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

    pub fn direction(&self, other: &Self) -> Vector {
        use num::signum;

        Vector::new(
            signum(other.x - self.x),
            signum(other.y - self.y),
        )
    }
}

impl From<&(isize, isize)> for Point {
    fn from(p: &(isize, isize)) -> Self {
        Self::new(p.0, p.1)
    }
}

impl From<(isize, isize)> for Point {
    fn from(p: (isize, isize)) -> Self {
        Point::from(&p)
    }
}

impl PartialEq<(isize, isize)> for Point {
    fn eq(&self, other: &(isize, isize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl From<&Direction> for Vector {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::North => Self::new(0, 1),
            Direction::South => Self::new(0, -1),
            Direction::East => Self::new(1, 0),
            Direction::West => Self::new(-1, 0),
        }
    }
}

impl From<Direction> for Vector {
    fn from(d: Direction) -> Vector {
        Vector::from(&d)
    }
}

#[derive(Debug)]
pub struct CoordGenerator {
    pub dir: Direction,
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
