#[aoc(day4, part1)]
pub fn part_1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (l, r) = line.split_once(',').unwrap();
        let (l_min, l_max) = l.split_once('-').unwrap();
        let (r_min, r_max) = r.split_once('-').unwrap();
        let l_min = l_min.parse::<u32>().unwrap();
        let l_max = l_max.parse::<u32>().unwrap();
        let r_min = r_min.parse::<u32>().unwrap();
        let r_max = r_max.parse::<u32>().unwrap();
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
        let (l, r) = line.split_once(',').unwrap();
        let (l_min, l_max) = l.split_once('-').unwrap();
        let (r_min, r_max) = r.split_once('-').unwrap();
        let l_min = l_min.parse::<u32>().unwrap();
        let l_max = l_max.parse::<u32>().unwrap();
        let r_min = r_min.parse::<u32>().unwrap();
        let r_max = r_max.parse::<u32>().unwrap();
        let l_range = l_min..=l_max;
        let r_range = r_min..=r_max;
        if l_range.contains(&r_min) || l_range.contains(&r_max)
            || r_range.contains(&l_min) || r_range.contains(&l_max)
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
"
    }

    #[allow(dead_code)]
    fn get_real_input<'a>() -> &'a str {
        include_str!("../input/2022/day4.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 2);
    }

    #[test]
    fn test_example_part2_() {
        assert_eq!(part_2(get_example_input()), 4);
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