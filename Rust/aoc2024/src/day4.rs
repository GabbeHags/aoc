use aoc_runner_derive::{aoc, aoc_generator};
use utils::grid::{Grid, ToGrid};

type ParseOut = Grid<u8>;

#[aoc_generator(day4)]
fn parse(input: &str) -> ParseOut {
    Grid::to_grid(input)
}

#[aoc(day4, part1)]
fn part1(input: &ParseOut) -> usize {
    // input.
    todo!()
}

#[aoc(day4, part2)]
fn part2(_input: &ParseOut) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_input() -> String {
        std::fs::read_to_string("./input/2024/day4.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    fn example_input() -> String {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .to_string()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&example_input())), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&example_input())), 0);
    }
}
