use std::collections::HashSet;


#[aoc(day3, part1)]
pub fn part_1(input: &str) -> u32 {
    input.lines().map(|line| {
        let storage: Vec<_> = line.bytes().collect();
        let cs = storage.split_at(line.len()/2);
        let intersection = cs.0.iter().find(|c| cs.1.contains(c)).unwrap();
        if ('a' as u8..='z' as u8).contains(&intersection) {
            (*intersection - 0x60) as u32
        } else {
            (*intersection - 0x40 + 26) as u32
        }
    }).sum()
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut groups = lines.chunks(3);
    let mut result = 0;
    while let Some([e1, e2, e3]) = groups.next() {
        let s1 = e1.bytes().collect::<HashSet<_>>();
        let s2 = e2.bytes().collect::<HashSet<_>>();
        let mut inter_s1_s2 = s1.intersection(&s2);
        let intersection = inter_s1_s2.find(|c| e3.as_bytes().contains(c)).unwrap();
        if ('a' as u8..='z' as u8).contains(&intersection) {
            result += (*intersection - 0x60) as u32
        } else {
            result += (*intersection - 0x40 + 26) as u32
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
