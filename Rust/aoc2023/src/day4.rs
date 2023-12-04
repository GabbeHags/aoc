use aoc_runner_derive::{aoc, aoc_generator};
use core::hash::Hash;
use smallvec::SmallVec;
use utils::counting_set::CountingSet;

type WinningVec = SmallVec<[u8; 10]>;
type MyVec = SmallVec<[u8; 25]>;

#[derive(Debug, Clone, Default)]
struct Card {
    card_num: usize,
    winning_numbers: WinningVec,
    my_numbers: MyVec,
}

impl Card {
    fn get_wins(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|num| self.my_numbers.contains(num))
            .count()
    }

    fn get_points(&self) -> usize {
        let wins = self.get_wins();
        if wins == 0 {
            0
        } else {
            2_usize.pow(wins as u32 - 1)
        }
    }
}

impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.card_num.hash(state);
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card_num == other.card_num
    }
}

impl Eq for Card {}

#[aoc_generator(day4, part1, parser)]
fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let line = &line[5..];
            let (winning, my) = line.split_once(" | ").unwrap();
            let winning_nums = winning.split_ascii_whitespace().skip(1).map(|num| {
                num.as_bytes()
                    .iter()
                    .fold(0, |acc, digit| acc * 10 + (digit - b'0'))
            });

            let my_nums = my.split_ascii_whitespace().map(|num| {
                num.as_bytes()
                    .iter()
                    .fold(0, |acc, digit| acc * 10 + (digit - b'0'))
            });
            let winning_numbers: WinningVec = winning_nums.collect();
            let my_numbers: MyVec = my_nums.collect();

            debug_assert!(!winning_numbers.spilled());
            debug_assert!(!my_numbers.spilled());
            Card {
                card_num: row + 1,
                winning_numbers,
                my_numbers,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    parse(input).iter().map(|card| card.get_points()).sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let parsed = parse(input);

    let mut counting_set = CountingSet::default();
    counting_set.extend(parsed.iter().cloned());

    let mut total = parsed.len();
    let mut card_index = 0;

    while !counting_set.is_empty() {
        let card = &parsed[card_index];
        if let Some(count) = counting_set.get(card) {
            for next_card in parsed[card.card_num..(card.card_num + card.get_wins())].iter() {
                total += count;
                counting_set.increase_with_many(next_card.clone(), count);
            }
        }
        counting_set.take(card);
        card_index += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim()
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day4.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(example_input()), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example_input()), 30);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(&real_input()), 26426);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(&real_input()), 6227972);
    }
}
