use std::ops::Range;

use aoc_runner_derive::aoc;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use smallvec::SmallVec;

#[aoc(day3, part1, first)]
fn part1_first(input: &str) -> usize {
    let map: Vec<_> = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            line.iter().fold(Vec::with_capacity(100), |mut acc, b| {
                acc.push(*b);
                acc
            })
        })
        .collect();
    let numbers: Vec<_> = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            let mut v: Vec<Range<usize>> = Vec::new();
            let mut start = 0;
            let mut end = 0;
            let mut in_number = false;
            for (index, b) in line.iter().enumerate() {
                end = index;
                if b.is_ascii_digit() {
                    if !in_number {
                        start = index;
                    }
                    in_number = true;
                } else {
                    if in_number {
                        debug_assert!(start.abs_diff(end) <= 3);
                        v.push(start..end)
                    }
                    in_number = false;
                }
            }
            if in_number {
                debug_assert!(start.abs_diff(end) <= 3);
                v.push(start..(end + 1))
            }
            v
        })
        .collect();
    let symbols: Vec<_> = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, b)| {
                    (!b.is_ascii_digit() && *b != b'.').then_some((row, col, *b as char))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // dbg!(&numbers);

    let mut sum_nums: Vec<usize> = Vec::new();
    for symbol_line in symbols {
        for (row, col, _symbol) in symbol_line {
            let top = numbers.get(row - 1).and_then(|nums| {
                let v = nums
                    .iter()
                    .filter_map(|range| {
                        (range.contains(&(col - 1))
                            || range.contains(&(col))
                            || range.contains(&(col + 1)))
                        .then_some((row - 1, range))
                    })
                    .collect::<Vec<_>>();
                (!v.is_empty()).then_some(v)
            });
            let mid = numbers.get(row).and_then(|nums| {
                let v = nums
                    .iter()
                    .filter_map(|range| {
                        (range.contains(&(col - 1))
                            || range.contains(&(col))
                            || range.contains(&(col + 1)))
                        .then_some((row, range))
                    })
                    .collect::<Vec<_>>();
                (!v.is_empty()).then_some(v)
            });
            let bot = numbers.get(row + 1).and_then(|nums| {
                let v = nums
                    .iter()
                    .filter_map(|range| {
                        (range.contains(&(col - 1))
                            || range.contains(&(col))
                            || range.contains(&(col + 1)))
                        .then_some((row + 1, range))
                    })
                    .collect::<Vec<_>>();
                (!v.is_empty()).then_some(v)
            });

            // dbg!(&top);
            // dbg!(&mid);
            // dbg!(&bot);

            if let Some(top) = top {
                for (row, range) in top {
                    sum_nums.push(
                        map[row][range.start..range.end]
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (index, digit)| {
                                10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
                            }),
                    )
                }
            }
            if let Some(mid) = mid {
                for (row, range) in mid {
                    sum_nums.push(
                        map[row][range.start..range.end]
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (index, digit)| {
                                10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
                            }),
                    )
                }
            }

            if let Some(bot) = bot {
                for (row, range) in bot {
                    sum_nums.push(
                        map[row][range.start..range.end]
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (index, digit)| {
                                10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
                            }),
                    )
                }
            }
        }
    }
    sum_nums
        .iter()
        .inspect(|num| debug_assert!(**num < 1000))
        .sum()
}

#[aoc(day3, part2, first)]
fn part2_first(input: &str) -> usize {
    let map: Vec<_> = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            line.iter().fold(Vec::with_capacity(100), |mut acc, b| {
                acc.push(*b);
                acc
            })
        })
        .collect();
    let numbers: Vec<_> = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| {
            let mut v: Vec<Range<usize>> = Vec::new();
            let mut start = 0;
            let mut end = 0;
            let mut in_number = false;
            for (index, b) in line.iter().enumerate() {
                end = index;
                if b.is_ascii_digit() {
                    if !in_number {
                        start = index;
                    }
                    in_number = true;
                } else {
                    if in_number {
                        debug_assert!(start.abs_diff(end) <= 3);
                        v.push(start..end)
                    }
                    in_number = false;
                }
            }
            if in_number {
                debug_assert!(start.abs_diff(end) <= 3);
                v.push(start..(end + 1))
            }
            v
        })
        .collect();
    let symbols: Vec<_> = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, b)| {
                    (!b.is_ascii_digit() && *b != b'.').then_some((row, col, *b as char))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum_nums: Vec<usize> = Vec::new();
    for symbol_line in symbols {
        for (row, col, symbol) in symbol_line {
            if symbol != '*' {
                continue;
            }
            let top = numbers.get(row - 1).and_then(|nums| {
                let v = nums
                    .iter()
                    .filter_map(|range| {
                        (range.contains(&(col - 1))
                            || range.contains(&(col))
                            || range.contains(&(col + 1)))
                        .then_some((row - 1, range))
                    })
                    .collect::<Vec<_>>();
                (!v.is_empty()).then_some(v)
            });
            let mid = numbers.get(row).and_then(|nums| {
                let v = nums
                    .iter()
                    .filter_map(|range| {
                        (range.contains(&(col - 1))
                            || range.contains(&(col))
                            || range.contains(&(col + 1)))
                        .then_some((row, range))
                    })
                    .collect::<Vec<_>>();
                (!v.is_empty()).then_some(v)
            });
            let bot = numbers.get(row + 1).and_then(|nums| {
                let v = nums
                    .iter()
                    .filter_map(|range| {
                        (range.contains(&(col - 1))
                            || range.contains(&(col))
                            || range.contains(&(col + 1)))
                        .then_some((row + 1, range))
                    })
                    .collect::<Vec<_>>();
                (!v.is_empty()).then_some(v)
            });

            let mut gear_ratio_numbers = SmallVec::<[usize; 2]>::new();
            if let Some(top) = top {
                for (row, range) in top {
                    gear_ratio_numbers.push(
                        map[row][range.start..range.end]
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (index, digit)| {
                                10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
                            }),
                    )
                }
            }
            if let Some(mid) = mid {
                for (row, range) in mid {
                    gear_ratio_numbers.push(
                        map[row][range.start..range.end]
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (index, digit)| {
                                10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
                            }),
                    )
                }
            }

            if let Some(bot) = bot {
                for (row, range) in bot {
                    gear_ratio_numbers.push(
                        map[row][range.start..range.end]
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (index, digit)| {
                                10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
                            }),
                    )
                }
            }
            debug_assert!(!gear_ratio_numbers.spilled());
            if gear_ratio_numbers.len() == 2 {
                sum_nums.push(gear_ratio_numbers.iter().product());
            }
        }
    }
    sum_nums.iter().sum()
}

type SymbolsVec = SmallVec<[(usize, usize, u8); 11]>;
type NumbersVec = SmallVec<[Range<usize>; 19]>;

fn get_marked_line_ranges(
    line_numbers: &[NumbersVec],
    row: usize,
    col: usize,
) -> Option<Vec<(usize, Range<usize>)>> {
    line_numbers.get(row).and_then(|nums| {
        let v = nums
            .iter()
            .filter_map(|range| {
                (range.contains(&(col - 1)) || range.contains(&(col)) || range.contains(&(col + 1)))
                    .then_some((row, range.clone()))
            })
            .collect::<Vec<_>>();
        (!v.is_empty()).then_some(v)
    })
}

fn parse_optimized(input: &str) -> (Vec<NumbersVec>, Vec<SymbolsVec>, Vec<Vec<u8>>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let schematic = input
        .as_bytes()
        .split(|b| *b == b'\n')
        .enumerate()
        .map(|(row, line)| {
            let mut res = Vec::with_capacity(100);
            let mut symbols_line = SymbolsVec::new();
            let mut v = NumbersVec::new();
            let mut start = 0;
            let mut end = 0;
            let mut in_number = false;
            for (col, b) in line.iter().enumerate() {
                res.push(*b);
                if !b.is_ascii_digit() && *b != b'.' {
                    symbols_line.push((row, col, *b))
                }
                end = col;
                if b.is_ascii_digit() {
                    if !in_number {
                        start = col;
                    }
                    in_number = true;
                } else {
                    if in_number {
                        debug_assert!(start.abs_diff(end) <= 3);
                        v.push(start..end)
                    }
                    in_number = false;
                }
            }
            if in_number {
                debug_assert!(start.abs_diff(end) <= 3);
                v.push(start..(end + 1))
            }
            debug_assert!(!v.spilled());
            debug_assert!(!symbols_line.spilled());
            symbols.push(symbols_line);
            numbers.push(v);
            res
        })
        .collect();
    (numbers, symbols, schematic)
}

fn parse_usize(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, digit)| {
            10_usize.pow(index as u32) * ((digit - b'0') as usize) + acc
        })
}

#[aoc(day3, part1, optimized)]
fn part1_optimized(input: &str) -> usize {
    let (numbers, symbols, schematic) = parse_optimized(input);

    let len = schematic.len();
    symbols
        .par_iter()
        .flatten_iter()
        .map(|(row, col, _symbol)| {
            ((row - 1).max(0)..=(row + 1).min(len)).fold(0, |mut acc, r| {
                let line = get_marked_line_ranges(&numbers, r, *col);
                if let Some(line) = line {
                    for (row, range) in line {
                        acc += parse_usize(&schematic[row][range.start..range.end]);
                    }
                }
                acc
            })
        })
        .sum()
}

#[aoc(day3, part2, optimized)]
fn part2_optimized(input: &str) -> usize {
    let (numbers, symbols, schematic) = parse_optimized(input);

    let len = schematic.len();
    symbols
        .par_iter()
        .flatten_iter()
        .filter_map(|(row, col, symbol)| {
            if *symbol != b'*' {
                None
            } else {
                let mut gear_ratio_numbers = SmallVec::<[usize; 2]>::new();
                ((row - 1).max(0)..=(row + 1).min(len)).for_each(|r| {
                    let line = get_marked_line_ranges(&numbers, r, *col);
                    if let Some(line) = line {
                        for (row, range) in line {
                            gear_ratio_numbers
                                .push(parse_usize(&schematic[row][range.start..range.end]));
                        }
                    }
                });
                debug_assert!(!gear_ratio_numbers.spilled());
                Some({
                    if gear_ratio_numbers.len() == 2 {
                        gear_ratio_numbers.iter().product::<usize>()
                    } else {
                        0
                    }
                })
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim()
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day3.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_first_example() {
        assert_eq!(part1_first(example_input()), 4361);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1_first(&real_input()), 540025);
    }

    #[test]
    fn part1_first_test1() {
        assert_eq!(
            part1_first(
                "
1.1
1#1
1.1"
                .trim()
            ),
            6
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_first(example_input()), 467835);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2_first(&real_input()), 84584891);
    }

    #[test]
    fn part1_optimized_example() {
        assert_eq!(part1_optimized(example_input()), 4361);
    }

    #[test]
    fn part1_optimized_real() {
        assert_eq!(part1_optimized(&real_input()), 540025);
    }

    #[test]
    fn part1_optimized_test1() {
        assert_eq!(
            part1_optimized(
                "
1.1
1#1
1.1"
                .trim()
            ),
            6
        );
    }

    #[test]
    fn part2_optimized_example() {
        assert_eq!(part2_optimized(example_input()), 467835);
    }

    #[test]
    fn part2_optimized_real() {
        assert_eq!(part2_optimized(&real_input()), 84584891);
    }
}
