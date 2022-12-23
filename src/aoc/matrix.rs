use crate::aoc::euclid::*;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix<T: Default + Ord> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<T>>,
}

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

    pub fn contains(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width as isize
            && p.y >= 0 && p.y < self.height as isize
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
