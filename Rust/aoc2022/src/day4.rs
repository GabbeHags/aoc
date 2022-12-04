type LMin = u8;
type LMax = u8;
type RMin = u8;
type RMax = u8;

#[inline(always)]
fn line_to_nums(line: &str) -> (LMin, LMax, RMin, RMax) {
    let mut line_bytes = line.bytes();
    (
        line_bytes
            .by_ref()
            .take_while(|c| *c != b'-')
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
        line_bytes
            .by_ref()
            .take_while(|c| *c != b',')
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
        line_bytes
            .by_ref()
            .take_while(|c| *c != b'-')
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
        line_bytes.fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
    )
}

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (l_min, l_max, r_min, r_max) = line_to_nums(line);
        if (l_min <= r_min && r_max <= l_max) || (r_min <= l_min && l_max <= r_max) {
            result += 1;
        }
    }
    result
}

#[aoc(day4, part2)]
pub fn part_2(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (l_min, l_max, r_min, r_max) = line_to_nums(line);
        if (l_min <= r_min && r_min <= l_max)
            || (r_min <= l_min && l_min <= r_max)
            || (l_min <= r_max && r_max <= l_max)
            || (r_min <= l_max && l_max <= r_max)
        {
            result += 1;
        }
    }
    result
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
1-98,2-97
"
    }

    #[allow(dead_code)]
    fn get_real_input<'a>() -> &'a str {
        include_str!("../input/2022/day4.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 3);
    }

    #[test]
    fn test_example_part2_() {
        assert_eq!(part_2(get_example_input()), 5);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 496);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(get_real_input()), 847);
    }
}
