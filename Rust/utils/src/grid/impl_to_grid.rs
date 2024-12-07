use super::grid::{Grid, ToGrid};

impl ToGrid<&str> for Grid<u8> {
    fn to_grid(input: &str) -> Grid<u8> {
        let byte_slice = input.as_bytes();
        let width = byte_slice.iter().position(|b| *b == b'\n').unwrap();
        let height = byte_slice.len() / width;

        Self::new(width, height, byte_slice)
    }
}

impl ToGrid<&[&[u8]]> for Grid<u8> {
    fn to_grid(input: &[&[u8]]) -> Grid<u8> {
        let width = input[0].len();
        let height = input.len();
        // let bytes = input.concat().to_vec();
        Self::new(width, height, &input.concat())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToGrid<[[u8; WIDTH]; HEIGHT]> for Grid<u8> {
    fn to_grid(input: [[u8; WIDTH]; HEIGHT]) -> Grid<u8> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(width, height, input.as_flattened())
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> ToGrid<&[&[u8; WIDTH]; HEIGHT]> for Grid<u8> {
    fn to_grid(input: &[&[u8; WIDTH]; HEIGHT]) -> Grid<u8> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(width, height, input.map(|a| *a).as_flattened())
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> ToGrid<[&[u8; WIDTH]; HEIGHT]> for Grid<u8> {
    fn to_grid(input: [&[u8; WIDTH]; HEIGHT]) -> Grid<u8> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(width, height, input.map(|a| *a).as_flattened())
    }
}
impl<const WIDTH: usize, const HEIGHT: usize> ToGrid<&[[u8; WIDTH]; HEIGHT]> for Grid<u8> {
    fn to_grid(input: &[[u8; WIDTH]; HEIGHT]) -> Grid<u8> {
        let width = WIDTH;
        let height = HEIGHT;
        Self::new(width, height, input.as_flattened())
    }
}

impl<const HEIGHT: usize> ToGrid<[&[u8]; HEIGHT]> for Grid<u8> {
    fn to_grid(input: [&[u8]; HEIGHT]) -> Grid<u8> {
        let width = input[HEIGHT].len();
        let height = HEIGHT;
        Self::new(width, height, &input.concat())
    }
}

impl<const WIDTH: usize> ToGrid<&[[u8; WIDTH]]> for Grid<u8> {
    fn to_grid(input: &[[u8; WIDTH]]) -> Grid<u8> {
        let width = WIDTH;
        let height = input.len();
        Self::new(width, height, input.as_flattened())
    }
}
