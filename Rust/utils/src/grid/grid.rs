use std::{
    fmt::Debug,
    iter::{repeat, zip},
    ops::Range,
};

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

impl<T: Debug> Grid<T> {
    fn map_row_indices<F, B>(
        &self,
        f: F,
    ) -> impl Iterator<Item = impl Iterator<Item = B>> + use<'_, F, B, T>
    where
        F: Copy + Fn(usize, usize) -> B,
    {
        (0..self.height).map(move |col| (0..self.width).map(move |row| f(row, col)))
    }

    fn map_column_indices<F, B>(
        &self,
        f: F,
    ) -> impl Iterator<Item = impl Iterator<Item = B>> + use<'_, F, B, T>
    where
        F: Copy + Fn(usize, usize) -> B,
    {
        (0..self.width).map(move |col| (0..self.height).map(move |row| f(row, col)))
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        debug_assert!(row < self.height);
        debug_assert!(col < self.width);
        &self.grid[self.width * row + col]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.grid.chunks_exact(self.width)
    }

    pub fn row(&self, row: usize) -> &[T] {
        self.rows().nth(row).unwrap()
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.map_column_indices(|row, col| self.get(row, col))
    }

    pub fn column(&self, col: usize) -> impl Iterator<Item = &T> {
        self.columns().nth(col).unwrap()
    }

    pub fn iter_all_surroundings(
        &self,
        size: usize,
    ) -> impl Iterator<Item = GridSurrounding<'_, T>> {
        self.map_row_indices(move |row, col| self.get_surrounding(row, col, size))
            .flatten()
    }

    pub fn get_surrounding(&self, row: usize, col: usize, size: usize) -> GridSurrounding<'_, T> {
        debug_assert!(row < self.height);
        debug_assert!(col < self.width);
        let rows = row.saturating_sub(size)..self.height.min(row.saturating_add(size + 1));
        let cols = col.saturating_sub(size)..self.width.min(col.saturating_add(size + 1));

        GridSurrounding::new(self.height, self.width, row, col, rows, cols, &self.grid)
    }
}

impl<T: Sync + Debug> Grid<T> {
    pub fn par_rows(&self) -> impl ParallelIterator<Item = &[T]> {
        self.grid.par_chunks_exact(self.width)
    }
    pub fn par_columns(&self) -> impl ParallelIterator<Item = impl Iterator<Item = &T>> {
        (0..self.width)
            .into_par_iter()
            .map(move |col| (0..self.height).map(move |row| self.get(row, col)))
    }
}

#[derive(Debug)]
pub struct GridSurrounding<'a, T> {
    _height: usize,
    width: usize,
    row: usize,
    col: usize,
    row_range: Range<usize>,
    col_range: Range<usize>,
    items: &'a [T],
}

impl<'a, T: Debug> GridSurrounding<'a, T> {
    fn new(
        height: usize,
        width: usize,
        row: usize,
        col: usize,
        row_range: Range<usize>,
        col_range: Range<usize>,
        items: &'a [T],
    ) -> Self {
        Self {
            _height: height,
            width,
            row,
            col,
            row_range,
            col_range,
            items,
        }
    }
    fn row_size(&self) -> usize {
        self.row_range.len()
    }
    fn col_size(&self) -> usize {
        self.row_range.len()
    }
    pub fn get_from_middle(&self, local_row: isize, local_col: isize) -> Option<&T> {
        let row = self.row.checked_add_signed(local_row)?;
        let col = self.col.checked_add_signed(local_col)?;
        if !(self.row_range.contains(&row) && self.col_range.contains(&col)) {
            return None;
        }
        Some(&self.items.chunks_exact(self.width).nth(row).unwrap()[col])
    }
    pub fn get_from_top_left(&self, local_row: usize, local_col: usize) -> Option<&T> {
        let row = self.row_range.start + local_row;
        let col = self.col_range.start + local_col;
        if !(self.row_range.contains(&row) && self.col_range.contains(&col)) {
            return None;
        }
        Some(&self.items.chunks_exact(self.width).nth(row).unwrap()[col])
    }
    pub fn get_square(&self) -> impl Iterator<Item = &[T]> {
        //#####
        //#####
        //#####
        //#####
        //#####
        self.items
            .chunks_exact(self.width)
            .skip(self.row_range.start)
            .take(self.row_size())
            .map(|row| &row[self.col_range.clone()])
    }
    pub fn get_plus(
        &self,
    ) -> PlusIter<
        impl Iterator<Item = &T>,
        impl Iterator<Item = &T>,
        impl Iterator<Item = &T>,
        impl Iterator<Item = &T>,
        &T,
    > {
        //  #
        //  #
        //#####
        //  #
        //  #

        let top_range = zip(
            (0..self.row_size() / 2 + 1)
                .map(|row| -(row as isize))
                .rev(),
            repeat(0),
        );
        let plus_iter_top = top_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        let left_range = zip(
            repeat(0),
            (0..self.col_size() / 2 + 1)
                .map(|row| -(row as isize))
                .rev(),
        );
        let plus_iter_left = left_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        let right_range = zip(
            repeat(0),
            (0..self.col_size() / 2 + 1).map(|row| (row as isize)),
        );
        let plus_iter_right = right_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        let bottom_range = zip(
            (0..self.row_size() / 2 + 1).map(|row| (row as isize)),
            repeat(0),
        );
        let plus_iter_bottom = bottom_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        PlusIter {
            plus_iter_top,
            plus_iter_left,
            plus_iter_right,
            plus_iter_bottom,
        }
    }
    pub fn get_cross(
        &self,
    ) -> CrossIter<
        impl Iterator<Item = &T>,
        impl Iterator<Item = &T>,
        impl Iterator<Item = &T>,
        impl Iterator<Item = &T>,
        &T,
    > {
        //#   #
        // # #
        //  #
        // # #
        //#   #
        let top_left_to_middle_range = zip(0..self.row_size() / 2 + 1, 0..self.col_size() / 2 + 1)
            .map(|(row, col)| (-(row as isize), -(col as isize)))
            .rev();
        dbg!(top_left_to_middle_range.clone().collect::<Vec<_>>());
        let cross_iter_top_left_to_middle =
            top_left_to_middle_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        let top_right_to_middle_range = zip(0..self.row_size() / 2 + 1, 0..self.col_size() / 2 + 1)
            .map(|(row, col)| (-(row as isize), col as isize))
            .rev();
        let cross_iter_top_right_to_middle =
            top_right_to_middle_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        let bottom_left_to_middle_range =
            zip(0..self.row_size() / 2 + 1, 0..self.col_size() / 2 + 1)
                .map(|(row, col)| (row as isize, -(col as isize)))
                .rev();
        let cross_iter_bottom_left_to_middle =
            bottom_left_to_middle_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        let bottom_right_to_middle_range =
            zip(0..self.row_size() / 2 + 1, 0..self.col_size() / 2 + 1)
                .map(|(row, col)| (row as isize, col as isize))
                .rev();
        let cross_iter_bottom_right_to_middle =
            bottom_right_to_middle_range.filter_map(|(row, col)| self.get_from_middle(row, col));

        CrossIter {
            cross_iter_top_left_to_middle,
            cross_iter_top_right_to_middle,
            cross_iter_bottom_left_to_middle,
            cross_iter_bottom_right_to_middle,
        }
    }
}

#[derive(Debug)]
pub struct PlusIter<T, L, R, B, Inner>
where
    T: Iterator<Item = Inner>,
    L: Iterator<Item = Inner>,
    R: Iterator<Item = Inner>,
    B: Iterator<Item = Inner>,
{
    pub plus_iter_top: T,
    pub plus_iter_left: L,
    pub plus_iter_right: R,
    pub plus_iter_bottom: B,
}
#[derive(Debug)]
pub struct CrossIter<T, L, R, B, Inner>
where
    T: Iterator<Item = Inner>,
    L: Iterator<Item = Inner>,
    R: Iterator<Item = Inner>,
    B: Iterator<Item = Inner>,
{
    pub cross_iter_top_left_to_middle: T,
    pub cross_iter_top_right_to_middle: L,
    pub cross_iter_bottom_left_to_middle: R,
    pub cross_iter_bottom_right_to_middle: B,
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
        let binding = grid.get_surrounding(1, 1, 0);
        let a = binding.get_square().collect::<Vec<_>>();
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
        let binding = grid.get_surrounding(1, 1, 1);
        let a = binding.get_square().collect::<Vec<_>>();
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
        let binding = grid.get_surrounding(0, 0, 1);
        let a = binding.get_square().collect::<Vec<_>>();
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
        let binding = grid.get_surrounding(4, 3, 1);
        let a = binding.get_square().collect::<Vec<_>>();
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
        let binding = grid.get_surrounding(2, 2, 2);
        let a = binding.get_square().collect::<Vec<_>>();
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

    #[test]
    fn test_plus_size_0() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(0, 0, 0);
        dbg!(&binding);
        let a = binding.get_plus();
        assert_eq!(
            [
                a.plus_iter_top.cloned().collect::<Vec<_>>(),
                a.plus_iter_left.cloned().collect::<Vec<_>>(),
                a.plus_iter_right.cloned().collect::<Vec<_>>(),
                a.plus_iter_bottom.cloned().collect::<Vec<_>>(),
            ],
            [vec![1], vec![1], vec![1], vec![1]]
        )
    }

    #[test]
    fn test_plus_size_1_left_edge() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(0, 0, 1);
        let a = binding.get_plus();
        assert_eq!(
            [
                a.plus_iter_top.cloned().collect::<Vec<_>>(),
                a.plus_iter_left.cloned().collect::<Vec<_>>(),
                a.plus_iter_right.cloned().collect::<Vec<_>>(),
                a.plus_iter_bottom.cloned().collect::<Vec<_>>(),
            ],
            [vec![1], vec![1], vec![1, 10], vec![1, 2]]
        )
    }
    #[test]
    fn test_plus_size_1_right_edge() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(4, 3, 1);
        dbg!(&binding);
        let a = binding.get_plus();
        assert_eq!(
            [
                dbg!(a.plus_iter_top.cloned().collect::<Vec<_>>()),
                dbg!(a.plus_iter_left.cloned().collect::<Vec<_>>()),
                dbg!(a.plus_iter_right.cloned().collect::<Vec<_>>()),
                dbg!(a.plus_iter_bottom.cloned().collect::<Vec<_>>()),
            ],
            [vec![4000, 5000], vec![500, 5000], vec![5000], vec![5000]]
        )
    }

    #[test]
    fn test_plus_size_2() {
        let input = [
            [1, 10, 100, 1000, 10000],
            [2, 20, 200, 2000, 20000],
            [3, 30, 300, 3000, 30000],
            [4, 40, 400, 4000, 40000],
            [5, 50, 500, 5000, 50000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(2, 2, 2);
        let a = binding.get_plus();
        assert_eq!(
            [
                a.plus_iter_top.cloned().collect::<Vec<_>>(),
                a.plus_iter_left.cloned().collect::<Vec<_>>(),
                a.plus_iter_right.cloned().collect::<Vec<_>>(),
                a.plus_iter_bottom.cloned().collect::<Vec<_>>(),
            ],
            [
                vec![100, 200, 300],
                vec![3, 30, 300],
                vec![300, 3000, 30000],
                vec![300, 400, 500]
            ]
        )
    }

    #[test]
    fn test_cross_size_0() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(0, 0, 0);
        dbg!(&binding);
        let a = binding.get_cross();
        assert_eq!(
            [
                a.cross_iter_top_left_to_middle.cloned().collect::<Vec<_>>(),
                a.cross_iter_top_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_left_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
            ],
            [vec![1], vec![1], vec![1], vec![1]]
        )
    }

    #[test]
    fn test_cross_size_1_left_edge() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(0, 0, 1);
        let a = binding.get_cross();
        assert_eq!(
            [
                a.cross_iter_top_left_to_middle.cloned().collect::<Vec<_>>(),
                a.cross_iter_top_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_left_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
            ],
            [vec![1], vec![1], vec![1], vec![20, 1]]
        )
    }
    #[test]
    fn test_cross_size_1_right_edge() {
        let input = [
            [1, 10, 100, 1000],
            [2, 20, 200, 2000],
            [3, 30, 300, 3000],
            [4, 40, 400, 4000],
            [5, 50, 500, 5000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(4, 3, 1);
        dbg!(&binding);
        let a = binding.get_cross();
        assert_eq!(
            [
                a.cross_iter_top_left_to_middle.cloned().collect::<Vec<_>>(),
                a.cross_iter_top_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_left_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
            ],
            [vec![400, 5000], vec![5000], vec![5000], vec![5000]]
        )
    }

    #[test]
    fn test_cross_size_2() {
        let input = [
            [1, 10, 100, 1000, 10000],
            [2, 20, 200, 2000, 20000],
            [3, 30, 300, 3000, 30000],
            [4, 40, 400, 4000, 40000],
            [5, 50, 500, 5000, 50000],
        ];
        let grid: Grid<u16> = Grid::to_grid(input);
        let binding = grid.get_surrounding(2, 2, 2);
        let a = binding.get_cross();

        assert_eq!(
            [
                a.cross_iter_top_left_to_middle.cloned().collect::<Vec<_>>(),
                a.cross_iter_top_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_left_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
                a.cross_iter_bottom_right_to_middle
                    .cloned()
                    .collect::<Vec<_>>(),
            ],
            [
                vec![1, 20, 300],
                vec![10000, 2000, 300],
                vec![5, 40, 300],
                vec![50000, 4000, 300]
            ]
        )
    }
}
