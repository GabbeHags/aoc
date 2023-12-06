use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use std::ops::Index;

#[derive(Debug)]
pub struct Grid<'a, T> {
    width: usize,
    _height: usize,
    slice: &'a [T],
}

impl<'a> Grid<'a, u8> {
    pub fn parse(input: &'a str) -> Self {
        let byte_slice = input.as_bytes();
        let width = byte_slice.iter().position(|b| *b == b'\n').unwrap();
        let height = byte_slice.len() / width;

        Self {
            width,
            _height: height,
            slice: byte_slice,
        }
    }
}

impl<'a, T> Grid<'a, T> {
    pub fn rows(&'a self) -> impl Iterator<Item = &[T]> + 'a {
        self.slice
            .chunks_exact(self.width + 1)
            .map(|s| &s[..self.width])
    }

    pub fn row(&self, row: usize) -> &[T] {
        self.rows().nth(row).unwrap()
    }
}

impl<'a, T: Sync> Grid<'a, T> {
    pub fn par_rows(&self) -> impl ParallelIterator<Item = &[T]> {
        self.slice
            .par_chunks_exact(self.width + 1)
            .map(|s| &s[..self.width])
    }
}

impl<'a, T> Index<(usize, usize)> for Grid<'a, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &'a Self::Output {
        &self.slice[self.width * index.0 + index.1]
    }
}
