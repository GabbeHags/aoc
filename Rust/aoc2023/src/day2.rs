use aoc_runner_derive::aoc;
use rayon::prelude::*;
use smallvec::SmallVec;
use std::hint::unreachable_unchecked;

type Sets = SmallVec<[ColorCount; 6]>;
#[derive(Default, Debug)]
struct Game {
    id: u8,
    sets: Sets,
}

impl Game {
    fn allowed<const MAX_RED: u8, const MAX_GREEN: u8, const MAX_BLUE: u8>(&self) -> bool {
        self.sets
            .iter()
            .all(|set| set.r <= MAX_RED && set.g <= MAX_GREEN && set.b <= MAX_BLUE)
    }

    fn get_max_set(&self) -> ColorCount {
        self.sets
            .iter()
            .fold(ColorCount::default(), |mut acc, set| {
                acc.r = acc.r.max(set.r);
                acc.g = acc.g.max(set.g);
                acc.b = acc.b.max(set.b);
                acc
            })
    }
}

#[derive(Default, Debug)]
struct ColorCount {
    r: u8,
    g: u8,
    b: u8,
}

const MAX_COLOR_COUNT: ColorCount = ColorCount {
    r: 12,
    g: 13,
    b: 14,
};

fn parse_game(line: &str) -> Game {
    let mut game = Game::default();
    let (game_id, sets) = line.split_once(": ").unwrap();
    let (_, id) = game_id.split_once(' ').unwrap();

    let id: u8 = id.parse().unwrap();
    game.id = id;

    game.sets = sets.split("; ").fold(Sets::new(), |mut acc_set, set| {
        acc_set.push(
            set.split(", ")
                .fold(ColorCount::default(), |mut acc, color| {
                    let (count, color_name) = color.split_once(' ').unwrap();
                    match color_name {
                        "red" => acc.r += count.parse::<u8>().unwrap(),
                        "green" => acc.g += count.parse::<u8>().unwrap(),
                        "blue" => acc.b += count.parse::<u8>().unwrap(),
                        _unreachable => unreachable!("{}", _unreachable),
                    };
                    acc
                }),
        );
        acc_set
    });
    debug_assert!(!game.sets.spilled());
    game
}

#[aoc(day2, part1, first)]
fn part1_first(input: &str) -> usize {
    input
        .lines()
        .map(parse_game)
        .filter_map(|game| {
            game.allowed::<{ MAX_COLOR_COUNT.r }, { MAX_COLOR_COUNT.g }, { MAX_COLOR_COUNT.b }>()
                .then_some(game.id as usize)
        })
        .sum()
}

#[aoc(day2, part2, first)]
fn part2_first(input: &str) -> usize {
    input
        .lines()
        .map(parse_game)
        .map(|game| {
            let set = game.get_max_set();
            set.r as usize * set.g as usize * set.b as usize
        })
        .sum()
}

type DigitsU8 = [Option<u8>; 3];

fn parse_u8(bytes_u8: impl Iterator<Item = u8>) -> u8 {
    const DIGITS: DigitsU8 = [None; 3];
    bytes_u8
        .enumerate()
        .fold(DIGITS, |mut digits, (index, digit)| {
            digits[index] = Some(digit);
            digits
        })
        .iter()
        .filter(|digit| digit.is_some())
        .rev()
        .enumerate()
        .fold(0, |id, (index, digit)| {
            10_u8.pow(index as u32) * unsafe { digit.unwrap_unchecked() } + id
        })
}

fn parse_game_optimized(line: &[u8]) -> Game {
    let mut game = Game::default();
    let mut line = line[5..].iter();

    game.id = parse_u8(
        line.by_ref()
            .take_while(|colon| **colon != b':')
            .map(|digit| (digit - b'0')),
    );

    'outer: loop {
        let mut color_count = ColorCount::default();
        'inner: loop {
            let count = parse_u8(
                line.by_ref()
                    .skip(1)
                    .take_while(|b| **b != b' ')
                    .map(|digit| (digit - b'0')),
            );

            match match line.next() {
                Some(b'r') => {
                    color_count.r = count;
                    line.by_ref().skip(2)
                }
                Some(b'g') => {
                    color_count.g = count;
                    line.by_ref().skip(4)
                }
                Some(b'b') => {
                    color_count.b = count;
                    line.by_ref().skip(3)
                }
                _ => unsafe { unreachable_unchecked() },
            }
            .next()
            {
                Some(b';') => break 'inner,
                None => {
                    game.sets.push(color_count);
                    break 'outer;
                }
                _ => { /* Do Nothing */ }
            }
        }
        game.sets.push(color_count);
    }
    debug_assert!(!game.sets.spilled());
    game
}

#[aoc(day2, part1, optimized)]
fn part1_optimized(input: &str) -> usize {
    input
        .as_bytes()
        .par_split(|b| *b == b'\n')
        .map(parse_game_optimized)
        .filter_map(|game| {
            game.allowed::<{ MAX_COLOR_COUNT.r }, { MAX_COLOR_COUNT.g }, { MAX_COLOR_COUNT.b }>()
                .then_some(game.id as usize)
        })
        .sum()
}

#[aoc(day2, part2, optimized)]
fn part2_optimized(input: &str) -> usize {
    input
        .as_bytes()
        .par_split(|b| *b == b'\n')
        .map(parse_game_optimized)
        .map(|game| {
            let set = game.get_max_set();
            set.r as usize * set.g as usize * set.b as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
        .trim()
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day2.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_first_example() {
        assert_eq!(part1_first(example_input()), 8);
    }

    #[test]
    fn part2_first_example() {
        assert_eq!(part2_first(example_input()), 2286);
    }

    #[test]
    fn part1_first_real() {
        assert_eq!(part1_first(&real_input()), 2162);
    }

    #[test]
    fn part2_first_real() {
        assert_eq!(part2_first(&real_input()), 72513);
    }

    #[test]
    fn part1_optimized_example() {
        assert_eq!(part1_optimized(example_input()), 8);
    }

    #[test]
    fn part2_optimized_example() {
        assert_eq!(part2_optimized(example_input()), 2286);
    }

    #[test]
    fn part1_optimized_real() {
        assert_eq!(part1_optimized(&real_input()), 2162);
    }

    #[test]
    fn part2_optimized_real() {
        assert_eq!(part2_optimized(&real_input()), 72513);
    }
}
