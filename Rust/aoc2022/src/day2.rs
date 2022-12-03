#[aoc(day2, part1)]
pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut chars = s.bytes();
            let left = chars.next().unwrap() - 0x40 - 1;
            chars.next();
            let right = chars.next().unwrap() - 0x40 - 0x17 - 1;
            let mut score = 1 + right;
            if left.abs_diff(right) == 0 {
                score += 3
            } else if (right + 1) % 3 != left {
                score += 6;
            }
            score as u32
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut chars = s.bytes();
            let left = chars.next().unwrap() - 0x40 - 1;
            chars.next();
            // lose = X = 0
            // draw = Y = 1
            // win  = Z = 2
            let goal = chars.next().unwrap() - 0x40 - 0x17 - 1;
            let score = match goal {
                0 => ((left + 2) % 3) + 1,
                1 => left + 3 + 1,
                2 => ((left + 1) % 3) + 6 + 1,
                _ => unreachable!(),
            };
            score as u32
        })
        .sum()
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input() -> &'static str {
        "A Y
B X
C Z
C X
B Z"
    }

    #[allow(dead_code)]
    fn get_real_input() -> &'static str {
        include_str!("../input/2022/day2.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(&get_example_input()), 31);
    }

    #[test]
    fn test_example_part2_() {
        assert_eq!(part_2(&get_example_input()), 23);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(&get_real_input()), 12740);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(&get_real_input()), 11980);
    }
}
