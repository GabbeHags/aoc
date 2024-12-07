use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Square,
    Plus,
    Cross,
}

pub trait ToGrid<T> {
    fn to_grid(input: T) -> Self;
}

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub(crate) fn new(width: usize, height: usize, grid: &[T]) -> Self {
        Self {
            width,
            height,
            grid: grid.to_vec(),
        }
    }
}

impl<T> Grid<T> {
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks_exact(self.width)
    }

    pub fn row(&self, row: usize) -> &[T] {
        self.rows().nth(row).unwrap()
    }

    pub fn get_surrounding(size: usize, shape: Shape) {
        todo!()
    }
}

impl<T: Sync> Grid<T> {
    pub fn par_rows(&self) -> impl ParallelIterator<Item = &[T]> {
        self.grid.par_chunks_exact(self.width)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[self.width * index.0 + index.1]
    }
}

#[cfg(test)]
mod tests_grid {
    use std::iter::zip;

    use rayon::iter::ParallelIterator;

    use super::{Grid, ToGrid};

    #[test]
    fn test_rows() {
        let input = [[1, 10], [2, 20], [3, 30], [4, 40], [5, 50]];
        let grid: Grid<u8> = Grid::to_grid(input);
        let rows: Vec<_> = grid.rows().collect();
        assert_eq!(input.len(), rows.len());
        for (inp, out) in zip(input, rows) {
            assert_eq!(inp, out)
        }
    }
    #[test]
    fn test_par_rows() {
        let input = [[1, 10], [2, 20], [3, 30], [4, 40], [5, 50]];
        let grid: Grid<u8> = Grid::to_grid(input);
        let rows: Vec<_> = grid.par_rows().collect();
        assert_eq!(input.len(), rows.len());
        for (inp, out) in zip(input, rows) {
            assert_eq!(inp, out)
        }
    }
}
