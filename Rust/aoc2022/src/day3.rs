use std::result;

trait Score {
    fn score(&self) -> u32;
}

impl Score for u8 {
    fn score(&self) -> u32 {
        if (b'a'..=b'z').contains(self) {
            (*self - b'a' + 1) as u32
        } else {
            (*self - b'A' + 27) as u32
        }
    }
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (c1, c2) = line.as_bytes().split_at(line.len() / 2);
        result += c1.iter().find(|c| c2.contains(c)).unwrap().score();
    }
    result
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u32 {
    let mut groups = input.lines();
    let mut result = 0;
    while let Some(e1) = groups.next() {
        let e2 = groups.next().unwrap().as_bytes();
        let e3 = groups.next().unwrap().as_bytes();
        result += e1
            .bytes()
            .find(|char| e2.contains(char) && e3.contains(char))
            .unwrap()
            .score();
    }
    result
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
    }

    #[allow(dead_code)]
    fn get_real_input() -> &'static str {
        include_str!("../input/2022/day3.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 157);
    }

    #[test]
    fn test_example_part2_() {
        assert_eq!(part_2(get_example_input()), 70);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 7878);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(get_real_input()), 2760);
    }
}
