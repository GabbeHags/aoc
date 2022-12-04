
type LMin = u8;
type LMax = u8;
type RMin = u8;
type RMax = u8;

#[inline(always)]
fn line_to_nums(input: &str) -> (LMin, LMax, RMin, RMax) {
    let (l_min, rest) = input.split_once('-').unwrap();
    let (l_max, rest) = rest.split_once(',').unwrap();
    let (r_min, r_max) = rest.split_once('-').unwrap();
    let nums = [l_min.as_bytes(), l_max.as_bytes(), r_min.as_bytes(), r_max.as_bytes()];
    let mut num = 0;
    let mut results = [0,0,0,0];
    for (index,n) in nums.iter().enumerate() {
        for (i, c) in n.iter().enumerate() {
            num += (*c - b'0') * 10_u8.pow((n.len()-i-1) as u32);
        }
        results[index] = num;
        num = 0;
    }
    (results[0], results[1], results[2], results[3])
}

#[aoc(day4, part1)]
pub fn part_1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (l_min, l_max, r_min, r_max) = line_to_nums(line);
        let l_range = l_min..=l_max;
        let r_range = r_min..=r_max;
        if l_range.contains(&r_min) && l_range.contains(&r_max)
            || r_range.contains(&l_min) && r_range.contains(&l_max)
        {
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
        let l_range = l_min..=l_max;
        let r_range = r_min..=r_max;
        if l_range.contains(&r_min)
            || l_range.contains(&r_max)
            || r_range.contains(&l_min)
            || r_range.contains(&l_max)
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
