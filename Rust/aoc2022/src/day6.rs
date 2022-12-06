// O(s.len()*window_len)
fn unique_chars_len(s: &str, window_len: usize) -> usize {
    let input_as_bytes_len = s.len();
    let input_as_bytes = s.as_bytes();

    // we can use this range because the input only contains letter in lowercase a to z
    const SET_SIZE: u8 = b'z' - b'a' + 1;

    type Set = [bool; SET_SIZE as usize];
    
    // i use an array as a set where the byte is the index to a
    // bool that have the meaning if we have seen this letter already
    let mut arr_set: Set = [false; SET_SIZE as usize];

    'outer: for i in window_len..input_as_bytes_len {
        for j in 0..window_len {
            if !arr_set[(input_as_bytes[j + i - window_len] - b'a') as usize] {
                arr_set[(input_as_bytes[j + i - window_len] - b'a') as usize] = true;
            } else {
                arr_set = [false; SET_SIZE as usize];
                continue 'outer;
            }
        }
        return i;
    }
    !0
}

// O(s.len()) but much higher constants for larger windows
fn unique_chars_len_2(s: &str, window_len: usize) -> usize {
    let input_as_bytes_len = s.len();
    let input_as_bytes = s.as_bytes();

    // we can use this range because the input only contains letter in lowercase a to z
    const SET_SIZE: u8 = b'z' - b'a' + 1;

    // i use an array as a set where the byte is the index to a
    // bool that have the meaning if we have seen this letter already
    type Set = [bool; SET_SIZE as usize];
    type SetCount = [u8; SET_SIZE as usize];

    let mut arr_set: Set = [false; SET_SIZE as usize];

    // this limits the amounts of duplicates in one window to 255;
    let mut arr_dup_count: SetCount = [0; SET_SIZE as usize];

    let mut index = 0;

    let mut unique_count = 0;

    while index < input_as_bytes_len {
        let set_index = (input_as_bytes[index] - b'a') as usize;
        if index < window_len {
            if !arr_set[set_index] {
                arr_set[set_index] = true;
                unique_count += 1;
            } else {
                arr_dup_count[set_index] += 1;
            }
        } else {
            let set_win_index = (input_as_bytes[index - window_len] - b'a') as usize;

            if !arr_set[set_index] {
                arr_set[set_index] = true;
                unique_count += 1;
            } else {
                arr_dup_count[set_index] += 1;
            }

            if arr_dup_count[set_win_index] == 0 {
                arr_set[set_win_index] = false;
                unique_count -= 1;
            } else {
                arr_dup_count[set_win_index] -= 1;
            }
        }

        index += 1;
        if unique_count == window_len {
            return index;
        }
    }

    !0
}

#[aoc(day6, part1, one)]
pub fn part_1(input: &str) -> usize {
    unique_chars_len(input, 4)
}

#[aoc(day6, part2, one)]
pub fn part_2(input: &str) -> usize {
    unique_chars_len(input, 14)
}

#[aoc(day6, part1, two)]
pub fn part_1_2(input: &str) -> usize {
    unique_chars_len_2(input, 4)
}

#[aoc(day6, part2, two)]
pub fn part_2_2(input: &str) -> usize {
    unique_chars_len_2(input, 14)
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

    // ---------------------------------------------------

    #[test]
    fn test_example_part1_0_2() {
        assert_eq!(part_1_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }
    #[test]
    fn test_example_part1_1_2() {
        assert_eq!(part_1_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }
    #[test]
    fn test_example_part1_2_2() {
        assert_eq!(part_1_2("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }
    #[test]
    fn test_example_part1_3_2() {
        assert_eq!(part_1_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }
    #[test]
    fn test_example_part1_4_2() {
        assert_eq!(part_1_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_example_part2_0_2() {
        assert_eq!(part_2_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }
    #[test]
    fn test_example_part2_1_2() {
        assert_eq!(part_2_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
    #[test]
    fn test_example_part2_2_2() {
        assert_eq!(part_2_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }
    #[test]
    fn test_example_part2_3_2() {
        assert_eq!(part_2_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
    #[test]
    fn test_example_part2_4_2() {
        assert_eq!(part_2_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_real_part1_2() {
        assert_eq!(part_1_2(get_real_input()), 1235);
    }

    #[test]
    fn test_real_part2_2() {
        assert_eq!(part_2_2(get_real_input()), 3051);
    }
}
