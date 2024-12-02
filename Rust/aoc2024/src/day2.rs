use arrayvec::ArrayVec;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::parse::ParseOps;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Report {
    levels: ArrayVec<Level, 8>,
}

impl Report {
    fn new(line: &[u8]) -> Self {
        let mut v: ArrayVec<Level, 8> = ArrayVec::new_const();
        for int in line.iter_unsigned::<u8>() {
            v.push(Level(int));
        }

        Self { levels: v }
    }

    fn is_safe_part1(&self) -> bool {
        self.levels
            .windows(3)
            .all(|x| x[0].is_safe_diff(&x[1], &x[2]))
    }

    fn is_safe_part2(&self) -> bool {
        (0..self.levels.len()).any(|i| {
            self.levels[..i]
                .iter()
                .chain(&self.levels[(i + 1)..])
                .tuple_windows::<(&Level, &Level, &Level)>()
                .all(|(x0, x1, x2)| x0.is_safe_diff(x1, x2))
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Level(u8);

impl Level {
    fn is_safe_diff(&self, other_1: &Level, other_2: &Level) -> bool {
        (self.0.abs_diff(other_1.0) <= 3)
            && (other_1.0.abs_diff(other_2.0) <= 3)
            && ((self.0 < other_1.0 && other_1.0 < other_2.0)
                || (self.0 > other_1.0 && other_1.0 > other_2.0))
    }
}

type ParseOut = Vec<Report>;

#[aoc_generator(day2)]
fn parse(input: &str) -> ParseOut {
    input
        .as_bytes()
        .split(|c| *c == b'\n')
        .map(Report::new)
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &ParseOut) -> usize {
    input.iter().filter(|x| x.is_safe_part1()).count()
}

#[aoc(day2, part2)]
fn part2(input: &ParseOut) -> usize {
    input.into_par_iter().filter(|x| x.is_safe_part2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_input() -> String {
        std::fs::read_to_string("./input/2024/day2.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    fn example_input() -> String {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&example_input())), 2);
    }
    #[test]
    fn part1_real() {
        assert_eq!(part1(&parse(&real_input())), 220);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&example_input())), 4);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(&parse(&real_input())), 296);
    }
}
