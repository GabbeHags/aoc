use std::iter::zip;

use aoc_runner_derive::aoc;
use smallvec::SmallVec;

type RaceVec = SmallVec<[Race; 4]>;

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn get_all_distances_count(&self) -> usize {
        // ht * (t - ht)        > r
        // ht * (t - ht) -r     > 0
        // ht * (t - ht) -r + 1 >= 0
        // ht*t - ht*ht -r + 1  >= 0
        //
        let a = -1_f64;
        let b = self.time as f64;
        let c = -(self.record as f64);

        let mut low = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil() as usize;
        let mut high = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor() as usize;

        if low * (b as usize - low) == self.record {
            low += 1;
        }
        if high * (b as usize - high) == self.record {
            high -= 1;
        }

        high - low + 1
    }
}

fn parse_part1(input: &str) -> RaceVec {
    let mut lines = input.lines();
    let times = lines.next().unwrap()[5..].trim().split_ascii_whitespace();

    let records = lines.next().unwrap()[9..].trim().split_ascii_whitespace();

    let race_vec: RaceVec = zip(times, records)
        .map(|(time, record)| Race {
            time: time
                .as_bytes()
                .iter()
                .fold(0, |acc, digit| acc * 10 + (*digit - b'0') as usize),
            record: record
                .as_bytes()
                .iter()
                .fold(0, |acc, digit| acc * 10 + (*digit - b'0') as usize),
        })
        .collect();

    debug_assert!(!race_vec.spilled());
    race_vec
}

fn parse_part2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines.next().unwrap()[5..]
        .as_bytes()
        .iter()
        .filter(|ch| ch.is_ascii_digit())
        .fold(0, |acc, digit| acc * 10 + (*digit - b'0') as usize);

    let record = lines.next().unwrap()[9..]
        .as_bytes()
        .iter()
        .filter(|ch| ch.is_ascii_digit())
        .fold(0, |acc, digit| acc * 10 + (*digit - b'0') as usize);

    Race { time, record }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let race_vec = parse_part1(input);
    race_vec
        .iter()
        .fold(1, |acc, race| acc * race.get_all_distances_count())
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let race = parse_part2(input);
    race.get_all_distances_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
Time:      7  15   30
Distance:  9  40  200
        "
        .trim()
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day6.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(example_input()), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example_input()), 71503);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(&real_input()), 781200);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(&real_input()), 49240091);
    }
}
