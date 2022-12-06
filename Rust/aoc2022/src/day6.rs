fn unique_chars_len(s: &str, len: usize) -> usize {
    let input_as_bytes_len = s.len();
    let input_as_bytes = s.as_bytes();
    let marker_len = len;
    const LOWERCASE_LETTERS_SIZE: u8 = b'z' - b'a' + 1; // we can use this range because the input only contains a to z
    let mut arr_set: [u8; LOWERCASE_LETTERS_SIZE as usize] = [0; LOWERCASE_LETTERS_SIZE as usize];
    'outer: for i in marker_len..input_as_bytes_len {
        for j in 0..marker_len {
            if arr_set[(input_as_bytes[j + i - marker_len] - b'a') as usize] == 0 {
                arr_set[(input_as_bytes[j + i - marker_len] - b'a') as usize] = 1;
            } else {
                arr_set = [0; LOWERCASE_LETTERS_SIZE as usize];
                continue 'outer;
            }
        }
        return i;
    }
    !0
}

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> usize {
    unique_chars_len(input, 4)
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> usize {
    unique_chars_len(input, 14)
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_real_input() -> &'static str {
        include_str!("../input/2022/day6.txt")
    }

    #[test]
    fn test_example_part1_0() {
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }
    #[test]
    fn test_example_part1_1() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }
    #[test]
    fn test_example_part1_2() {
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }
    #[test]
    fn test_example_part1_3() {
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }
    #[test]
    fn test_example_part1_4() {
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_example_part2_0() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }
    #[test]
    fn test_example_part2_1() {
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
    #[test]
    fn test_example_part2_2() {
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }
    #[test]
    fn test_example_part2_3() {
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
    #[test]
    fn test_example_part2_4() {
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 1235);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(get_real_input()), 3051);
    }
}
