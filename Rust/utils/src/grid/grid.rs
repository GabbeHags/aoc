use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};

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
    pub fn get(&self, row: usize, col: usize) -> &T {
        assert!(row <= self.height);
        assert!(col <= self.width);
        &self.grid[self.width * row + col]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks_exact(self.width)
    }

    pub fn row(&self, row: usize) -> &[T] {
        self.rows().nth(row).unwrap()
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.width).map(move |col| (0..self.height).map(move |row| self.get(row, col)))
    }

    pub fn column(&self, col: usize) -> impl Iterator<Item = &T> {
        self.columns().nth(col).unwrap()
    }

    pub fn get_surrounding(
        &self,
        row: usize,
        col: usize,
        size: usize,
    ) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        assert!(row < self.height);
        assert!(col < self.width);
        let rows = row.saturating_sub(size)..self.height.min(row.saturating_add(size + 1));
        let cols = col.saturating_sub(size)..self.width.min(col.saturating_add(size + 1));

        rows.map(move |r| cols.clone().map(move |c| self.get(r, c)))
    }
}

impl<T: Sync> Grid<T> {
    pub fn par_rows(&self) -> impl ParallelIterator<Item = &[T]> {
        self.grid.par_chunks_exact(self.width)
    }
    pub fn par_columns(&self) -> impl ParallelIterator<Item = impl Iterator<Item = &T>> {
        (0..self.width)
            .into_par_iter()
            .map(move |col| (0..self.height).map(move |row| self.get(row, col)))
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
    fn test_row() {
        let input = [[1, 10], [2, 20], [3, 30], [4, 40], [5, 50]];
        let grid: Grid<u8> = Grid::to_grid(input);

        for (index, row) in input.iter().enumerate() {
            assert_eq!(row, grid.row(index))
        }
    }

    #[test]
    fn test_columns() {
        let input = [[1, 10], [2, 20], [3, 30], [4, 40], [5, 50]];
        let output = [[1_u8, 2, 3, 4, 5], [10, 20, 30, 40, 50]];
        let grid: Grid<u8> = Grid::to_grid(input);
        let columns: Vec<_> = grid.columns().collect();
        assert_eq!(output.len(), columns.len());
        for (inp, out) in zip(output, columns) {
            assert_eq!(inp.to_vec(), out.cloned().collect::<Vec<_>>())
        }
    }

    #[test]
    fn test_column() {
        let input = [[1, 10], [2, 20], [3, 30], [4, 40], [5, 50]];
        let output = [[1_u8, 2, 3, 4, 5], [10, 20, 30, 40, 50]];
        let grid: Grid<u8> = Grid::to_grid(input);

        for (index, row) in output.iter().enumerate() {
            assert_eq!(
                row.to_vec(),
                grid.column(index).cloned().collect::<Vec<_>>()
            )
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

    #[test]
    fn test_get_surrounding_size_0() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let a = grid
            .get_surrounding(1, 1, 0)
            .map(|x| x.cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(a, vec![vec![20]])
    }
    #[test]
    fn test_get_surrounding_size_1() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let a = grid
            .get_surrounding(1, 1, 1)
            .map(|x| x.cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            a,
            vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]]
        )
    }
    #[test]
    fn test_get_surrounding_size_1_left_edge() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let a = grid
            .get_surrounding(0, 0, 1)
            .map(|x| x.cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(a, vec![vec![1, 10], vec![2, 20]])
    }
    #[test]
    fn test_get_surrounding_size_1_right_edge() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let a = grid
            .get_surrounding(4, 3, 1)
            .map(|x| x.cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(a, vec![vec![400, 4000], vec![500, 5000]])
    }
    #[test]
    fn test_get_surrounding_size_2() {
        let input = [
            [1, 10, 100, 1000, 10000],
            [2, 20, 200, 2000, 20000],
            [3, 30, 300, 3000, 30000],
            [4, 40, 400, 4000, 40000],
            [5, 50, 500, 5000, 50000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let a = grid
            .get_surrounding(2, 2, 2)
            .map(|x| x.cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            a,
            vec![
                vec![1, 10, 100, 1000, 10000],
                vec![2, 20, 200, 2000, 20000],
                vec![3, 30, 300, 3000, 30000],
                vec![4, 40, 400, 4000, 40000],
                vec![5, 50, 500, 5000, 50000],
            ]
        )
    }
}
