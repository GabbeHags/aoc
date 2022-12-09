struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn build_grid(input: &str) -> Self {
        let mut rows = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for n in line.bytes() {
                row.push(n & 0b00001111);
            }
            rows.push(row);
        }
        Self { grid: rows }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        self.is_visible_from(Dir::N, (x, y), self.grid[y][x], 0).0
            || self.is_visible_from(Dir::W, (x, y), self.grid[y][x], 0).0
            || self.is_visible_from(Dir::S, (x, y), self.grid[y][x], 0).0
            || self.is_visible_from(Dir::E, (x, y), self.grid[y][x], 0).0
    }

    fn is_visible_from(
        &self,
        dir: Dir,
        (x, y): (usize, usize),
        tree_size: u8,
        steps: usize,
    ) -> (bool, usize) {
        match dir {
            Dir::N => {
                if y == 0 {
                    (true, steps)
                } else if self.grid[y - 1][x] < tree_size {
                    self.is_visible_from(dir, (x, y - 1), tree_size, steps + 1)
                } else {
                    (false, steps + 1)
                }
            }
            Dir::W => {
                if x == 0 {
                    (true, steps)
                } else if self.grid[y][x - 1] < tree_size {
                    self.is_visible_from(dir, (x - 1, y), tree_size, steps + 1)
                } else {
                    (false, steps + 1)
                }
            }
            Dir::S => {
                if y == self.grid.len() - 1 {
                    (true, steps)
                } else if self.grid[y + 1][x] < tree_size {
                    self.is_visible_from(dir, (x, y + 1), tree_size, steps + 1)
                } else {
                    (false, steps + 1)
                }
            }
            Dir::E => {
                if x == self.grid[y].len() - 1 {
                    (true, steps)
                } else if self.grid[y][x + 1] < tree_size {
                    self.is_visible_from(dir, (x + 1, y), tree_size, steps + 1)
                } else {
                    (false, steps + 1)
                }
            }
        }
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> usize {
        let (_, steps_n) = self.is_visible_from(Dir::N, (x, y), self.grid[y][x], 0);
        let (_, steps_w) = self.is_visible_from(Dir::W, (x, y), self.grid[y][x], 0);
        let (_, steps_s) = self.is_visible_from(Dir::S, (x, y), self.grid[y][x], 0);
        let (_, steps_e) = self.is_visible_from(Dir::E, (x, y), self.grid[y][x], 0);
        steps_n * steps_w * steps_s * steps_e
    }

    fn size(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }
}

enum Dir {
    N,
    W,
    S,
    E,
}

#[aoc(day8, part1)]
pub fn part_1(input: &str) -> usize {
    let grid = Grid::build_grid(input);
    let (x_len, y_len) = grid.size();
    let mut visible_count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            if grid.is_visible(x, y) {
                visible_count += 1;
            }
        }
    }

    visible_count
}

#[aoc(day8, part2)]
pub fn part_2(input: &str) -> usize {
    let grid = Grid::build_grid(input);
    grid.grid
        .iter()
        .enumerate()
        .map(|(idx_y, trees)| {
            trees
                .iter()
                .enumerate()
                .map(|(idx_x, _tree)| grid.get_scenic_score(idx_x, idx_y))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "
30373
25512
65332
33549
35390
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_real_input<'a>() -> &'a str {
        include_str!("../input/2022/day8.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 21);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part_2(get_example_input()), 8);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 1845);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(get_real_input()), 230112);
    }
}
