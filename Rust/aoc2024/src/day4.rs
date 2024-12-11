use aoc_runner_derive::{aoc, aoc_generator};
use utils::grid::{Grid, ToGrid};

type ParseOut = Grid<u8>;

#[aoc_generator(day4)]
fn parse(input: &str) -> ParseOut {
    Grid::to_grid(input)
}

fn iter_eq<'a>(iter: impl Iterator<Item = &'a u8>, word: &str) -> bool {
    // dbg!(iter.map(|x| *x as char).collect_vec())
    //     .iter()
    //     .map(|x| *x as u8)
    //     .eq(word.as_bytes().iter().cloned())

    iter.eq(word.as_bytes())
}

#[aoc(day4, part1)]
fn part1(input: &ParseOut) -> usize {
    const WORD: &str = "XMAS";
    const WORD_REV: &str = "SAMX";
    let mut sum = 0;
    for surrounding in input.iter_all_surroundings(WORD.len() - 1) {
        if surrounding.get_from_middle(0, 0).unwrap() != &b'X' {
            continue;
        }
        let plus = surrounding.get_plus();
        let plus_rev = surrounding.get_plus();
        let cross = surrounding.get_cross();
        let cross_rev = surrounding.get_cross();
        let eq_arr = [
            iter_eq(plus.plus_iter_bottom, WORD),
            iter_eq(plus.plus_iter_left, WORD),
            iter_eq(plus.plus_iter_right, WORD),
            iter_eq(plus.plus_iter_top, WORD),
            iter_eq(plus_rev.plus_iter_bottom, WORD_REV),
            iter_eq(plus_rev.plus_iter_left, WORD_REV),
            iter_eq(plus_rev.plus_iter_right, WORD_REV),
            iter_eq(plus_rev.plus_iter_top, WORD_REV),
            iter_eq(cross.cross_iter_bottom_left_to_middle, WORD),
            iter_eq(cross.cross_iter_bottom_right_to_middle, WORD),
            iter_eq(cross.cross_iter_top_left_to_middle, WORD),
            iter_eq(cross.cross_iter_top_right_to_middle, WORD),
            iter_eq(cross_rev.cross_iter_bottom_left_to_middle, WORD_REV),
            iter_eq(cross_rev.cross_iter_bottom_right_to_middle, WORD_REV),
            iter_eq(cross_rev.cross_iter_top_left_to_middle, WORD_REV),
            iter_eq(cross_rev.cross_iter_top_right_to_middle, WORD_REV),
        ];

        sum += eq_arr.iter().map(|x| *x as usize).sum::<usize>();
    }

    sum
}

#[aoc(day4, part2)]
fn part2(input: &ParseOut) -> usize {
    const WORD: &str = "MAS";
    const WORD_REV: &str = "SAM";
    let mut sum = 0;
    for surrounding in input.iter_all_surroundings(1) {
        if surrounding.get_from_middle(0, 0).unwrap() != &b'A' {
            continue;
        }
        let cross = surrounding.get_cross();
        let cross_rev = surrounding.get_cross();

        let top_left_to_bottom_right = cross
            .cross_iter_top_left_to_middle
            .chain(cross.cross_iter_bottom_right_to_middle.rev().skip(1));
        if top_left_to_bottom_right.size_hint().1.unwrap() != WORD.len() {
            continue;
        }

        let top_right_to_bottom_left = cross
            .cross_iter_top_right_to_middle
            .chain(cross.cross_iter_bottom_left_to_middle.rev().skip(1));

        if top_right_to_bottom_left.size_hint().1.unwrap() != WORD.len() {
            continue;
        }

        let rev_top_left_to_bottom_right = cross_rev
            .cross_iter_top_left_to_middle
            .chain(cross_rev.cross_iter_bottom_right_to_middle.rev().skip(1));

        let rev_top_right_to_bottom_left = cross_rev
            .cross_iter_top_right_to_middle
            .chain(cross_rev.cross_iter_bottom_left_to_middle.rev().skip(1));

        let top_left_bottom_right = iter_eq(top_left_to_bottom_right, WORD);
        let top_right_bottom_left = iter_eq(top_right_to_bottom_left, WORD);
        let rev_top_left_bottom_right = iter_eq(rev_top_left_to_bottom_right, WORD_REV);
        let rev_top_right_bottom_left = iter_eq(rev_top_right_to_bottom_left, WORD_REV);

        sum += ((rev_top_right_bottom_left || top_right_bottom_left)
            && (rev_top_left_bottom_right || top_left_bottom_right)) as usize
    }

    sum
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
        assert_eq!(part1(&parse(&example_input())), 18);
    }
    #[test]
    fn part1_real() {
        assert_eq!(part1(&parse(&real_input())), 2545);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&example_input())), 9);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(&parse(&real_input())), 1886);
    }
}
