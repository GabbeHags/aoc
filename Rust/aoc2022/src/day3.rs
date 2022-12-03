#[aoc(day3, part1)]
pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|e| {
            let (c1, c2) = e.as_bytes().split_at(e.len() / 2);
            let intersection = c1.iter().find(|c| c2.contains(c)).unwrap();
            if ('a' as u8..='z' as u8).contains(&intersection) {
                *intersection as u32 - 0x60
            } else {
                *intersection as u32 - 0x40 + 26
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u32 {
    let mut groups = input.lines();
    let mut result = 0;
    while let Some(e1) = groups.next() {
        let e2 = groups.next().unwrap();
        let e3 = groups.next().unwrap();
        let intersection = e1
            .bytes()
            .find(|char| e2.as_bytes().contains(char) && e3.as_bytes().contains(char))
            .unwrap();
        if ('a' as u8..='z' as u8).contains(&intersection) {
            result += intersection as u32 - 0x60;
        } else {
            result += intersection as u32 - 0x40 + 26 as u32;
        }
    }
    result
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input() -> &'static str {
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
        assert_eq!(part_1(&get_example_input()), 157);
    }

    #[test]
    fn test_example_part2_() {
        assert_eq!(part_2(&get_example_input()), 70);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(&get_real_input()), 7878);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(&get_real_input()), 2760);
    }
}
