use super::grid::{Grid, ToGrid};

impl ToGrid<&str> for Grid<u8> {
    fn to_grid(input: &str) -> Grid<u8> {
        let byte_slice = input.as_bytes();
        let width = byte_slice.iter().position(|b| *b == b'\n').unwrap();
        let height = byte_slice.len() / width;

        Self::new(
            width,
            height,
            byte_slice.iter().filter(|b| **b != b'\n').cloned(),
        )
    }
}
impl ToGrid<&str> for Grid<char> {
    fn to_grid(input: &str) -> Grid<char> {
        let byte_slice = input.as_bytes();
        let width = byte_slice.iter().position(|b| *b == b'\n').unwrap();
        let height = byte_slice.len() / width;

        Self::new(
            width,
            height,
            byte_slice
                .iter()
                .filter(|b| **b != b'\n')
                .map(|b| *b as char),
        )
    }
}

impl<T: Clone> ToGrid<&[&[T]]> for Grid<T> {
    fn to_grid(input: &[&[T]]) -> Grid<T> {
        let width = input[0].len();
        let height = input.len();
        Self::new(width, height, input.concat().into_iter())
    }
}

impl<T: Clone, const WIDTH: usize, const HEIGHT: usize> ToGrid<[[T; WIDTH]; HEIGHT]> for Grid<T> {
    fn to_grid(input: [[T; WIDTH]; HEIGHT]) -> Grid<T> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(width, height, input.as_flattened().iter().cloned())
    }
}
impl<T: Clone, const WIDTH: usize, const HEIGHT: usize> ToGrid<&[&[T; WIDTH]; HEIGHT]> for Grid<T> {
    fn to_grid(input: &[&[T; WIDTH]; HEIGHT]) -> Grid<T> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(
            width,
            height,
            input.map(|a| a.clone()).as_flattened().iter().cloned(),
        )
    }
}
impl<T: Clone, const WIDTH: usize, const HEIGHT: usize> ToGrid<[&[T; WIDTH]; HEIGHT]> for Grid<T> {
    fn to_grid(input: [&[T; WIDTH]; HEIGHT]) -> Grid<T> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(
            width,
            height,
            input.map(|a| a.clone()).as_flattened().iter().cloned(),
        )
    }
}
impl<T: Clone, const WIDTH: usize, const HEIGHT: usize> ToGrid<&[[T; WIDTH]; HEIGHT]> for Grid<T> {
    fn to_grid(input: &[[T; WIDTH]; HEIGHT]) -> Grid<T> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(width, height, input.as_flattened().iter().cloned())
    }
}

impl<T: Clone, const HEIGHT: usize> ToGrid<[&[T]; HEIGHT]> for Grid<T> {
    fn to_grid(input: [&[T]; HEIGHT]) -> Grid<T> {
        let width = input[HEIGHT].len();
        let height = HEIGHT;
        Self::new(width, height, input.concat().into_iter())
    }
}

impl<T: Clone, const WIDTH: usize> ToGrid<&[[T; WIDTH]]> for Grid<T> {
    fn to_grid(input: &[[T; WIDTH]]) -> Grid<T> {
        let width = WIDTH;
        let height = input.len();
        Self::new(width, height, input.as_flattened().iter().cloned())
    }
}
