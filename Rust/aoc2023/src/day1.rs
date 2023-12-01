use aoc_runner_derive::aoc;
use rayon::prelude::*;

#[aoc(day1, part1, first)]
fn part1_first(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let i1 = line.find(|d: char| d.is_ascii_digit()).unwrap();
        let i2 = line.rfind(|d: char| d.is_ascii_digit()).unwrap();
        let bytes = line.as_bytes();
        total += ((bytes[i1] - b'0') * 10 + (bytes[i2] - b'0')) as usize;
    }
    total
}

#[aoc(day1, part2, first)]
fn part2_first(input: &str) -> usize {
    const STR_NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let mut start_index = usize::MAX;
            let mut start_num = 0;
            let mut end_index = usize::MIN;
            let mut end_num = 0;
            for (num_minus_one, num_text) in STR_NUMS.into_iter().enumerate() {
                let num = num_minus_one + 1;
                if let Some(num_index) = line.find(num_text) {
                    if num_index <= start_index {
                        start_index = num_index;
                        start_num = num;
                    }
                }
                if let Some(num_index) = line.rfind(num_text) {
                    if num_index >= end_index {
                        end_index = num_index;
                        end_num = num;
                    }
                }
            }

            if let Some(index) = line.find(|d: char| d.is_ascii_digit()) {
                if index <= start_index {
                    start_num = (line.as_bytes()[index] - b'0') as usize;
                }
            }

            if let Some(index) = line.rfind(|d: char| d.is_ascii_digit()) {
                if index >= end_index {
                    end_num = (line.as_bytes()[index] - b'0') as usize;
                }
            }
            start_num * 10 + end_num
        })
        .sum()
}

const BYTES_NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn line_to_digits<const PART2: bool>(line: &[u8]) -> usize {
    let mut nums = (0..line.len()).filter_map(|index| match line[index] {
        b'1'..=b'9' => Some((line[index] - b'0') as usize),
        _ if PART2 => BYTES_NUMS
            .iter()
            .enumerate()
            .find_map(|(num_index, num)| line[index..].starts_with(num).then_some(num_index + 1)),
        _ => None,
    });
    let first = nums.next().unwrap();
    let last = nums.next_back().unwrap_or(first);

    first * 10 + last
}

#[aoc(day1, part1, optimized)]
fn part1_optimized(input: &str) -> usize {
    input
        .as_bytes()
        .par_split(|b| *b == b'\n')
        .map(line_to_digits::<false>)
        .sum()
}

#[aoc(day1, part2, optimized)]
fn part2_optimized(input: &str) -> usize {
    input
        .as_bytes()
        .par_split(|b| *b == b'\n')
        .map(line_to_digits::<true>)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_p1() -> &'static str {
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    }

    fn example_input_p2() -> &'static str {
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day1.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_first_example() {
        assert_eq!(part1_first(example_input_p1()), 142);
    }

    #[test]
    fn part1_optimized_example() {
        assert_eq!(part1_optimized(example_input_p1()), 142);
    }

    #[test]
    fn part1_first_real() {
        assert_eq!(part1_first(&real_input()), 57346);
    }

    #[test]
    fn part1_optimized_real() {
        assert_eq!(part1_optimized(&real_input()), 57346);
    }

    #[test]
    fn part2_first_example() {
        assert_eq!(part2_first(example_input_p2()), 281);
    }

    #[test]
    fn part2_optimized_example() {
        assert_eq!(part2_optimized(example_input_p2()), 281);
    }

    #[test]
    fn part2_first_real() {
        assert_eq!(part2_first(&real_input()), 57345);
    }

    #[test]
    fn part2_optimized_real() {
        assert_eq!(part2_optimized(&real_input()), 57345);
    }
}
