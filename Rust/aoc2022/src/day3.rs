trait Prio {
    fn prio(&self) -> u32;
}

impl Prio for u8 {
    fn prio(&self) -> u32 {
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
        let mut c1_bits: u64 = 0;
        let mut c2_bits: u64 = 0;
        let (c1, c2) = line.as_bytes().split_at(line.len() / 2);
        for i in 0..(line.len() / 2) {
            c1_bits |= 1 << (c1[i] - b'A');
            c2_bits |= 1 << (c2[i] - b'A');
        }
        let intersections = c1_bits & c2_bits;
        result += ((64 - intersections.leading_zeros()) as u8 + b'A' - 1).prio();
    }
    result
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> u32 {
    let mut groups = input.lines();
    let mut result = 0;
    while let Some(e1) = groups.next() {
        let mut e1_bits: u64 = 0;
        let mut e2_bits: u64 = 0;
        let mut e3_bits: u64 = 0;
        for c in e1.as_bytes() {
            e1_bits |= 1 << (c - b'A');
        }
        for c in groups.next().unwrap().as_bytes() {
            e2_bits |= 1 << (c - b'A');
        }
        for c in groups.next().unwrap().as_bytes() {
            e3_bits |= 1 << (c - b'A');
        }
        let intersections = e1_bits & e2_bits & e3_bits;
        result += ((64 - intersections.leading_zeros()) as u8 + b'A' - 1).prio();
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
