// O(s.len()) but much higher constants for larger windows
fn find_unique_window<const WIN_LEN: usize>(s: &str) -> usize {
    let input_as_bytes_len = s.len();
    let input_as_bytes = s.as_bytes();

    // we can use this range because the input only contains letter in lowercase a to z
    const SET_SIZE: u8 = b'z' - b'a' + 1;

    type Set = [u8; SET_SIZE as usize];

    // i use an array as a set where the byte is the index to a
    // bool that have the meaning if we have seen this letter already
    let mut arr_set: Set = [0; SET_SIZE as usize];

    let mut index = 0;

    let mut unique_count = 0;

    let mut set_index;

    for _ in 0..WIN_LEN {
        set_index = (input_as_bytes[index] - b'a') as usize;

        arr_set[set_index] += 1;
        if arr_set[set_index] == 1 {
            unique_count += 1;
        }
        index += 1;
    }

    let mut set_win_index;

    while index < input_as_bytes_len {
        set_index = (input_as_bytes[index] - b'a') as usize;

        arr_set[set_index] += 1;
        if arr_set[set_index] == 1 {
            unique_count += 1;
        }

        set_win_index = (input_as_bytes[index - WIN_LEN] - b'a') as usize;

        arr_set[set_win_index] -= 1;
        if arr_set[set_win_index] == 0 {
            unique_count -= 1;
        }

        index += 1;
        if unique_count == WIN_LEN {
            return index;
        }
    }

    !0
}

#[aoc(day6, part1, one)]
pub fn part_1(input: &str) -> usize {
    find_unique_window::<4>(input)
}

#[aoc(day6, part2, one)]
pub fn part_2(input: &str) -> usize {
    find_unique_window::<14>(input)
}

// #[aoc(day6, part1, two)]
// pub fn part_1_2(input: &str) -> usize {
//     sliding_window(input, 4)
// }

// #[aoc(day6, part2, two)]
// pub fn part_2_2(input: &str) -> usize {
//     sliding_window(input, 14)
// }

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

    // #[test]
    // fn test_example_part1_0_2() {
    //     assert_eq!(part_1_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    // }
    // #[test]
    // fn test_example_part1_1_2() {
    //     assert_eq!(part_1_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    // }
    // #[test]
    // fn test_example_part1_2_2() {
    //     assert_eq!(part_1_2("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    // }
    // #[test]
    // fn test_example_part1_3_2() {
    //     assert_eq!(part_1_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    // }
    // #[test]
    // fn test_example_part1_4_2() {
    //     assert_eq!(part_1_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    // }

    // #[test]
    // fn test_example_part2_0_2() {
    //     assert_eq!(part_2_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    // }
    // #[test]
    // fn test_example_part2_1_2() {
    //     assert_eq!(part_2_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    // }
    // #[test]
    // fn test_example_part2_2_2() {
    //     assert_eq!(part_2_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    // }
    // #[test]
    // fn test_example_part2_3_2() {
    //     assert_eq!(part_2_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    // }
    // #[test]
    // fn test_example_part2_4_2() {
    //     assert_eq!(part_2_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    // }

    // #[test]
    // fn test_real_part1_2() {
    //     assert_eq!(part_1_2(get_real_input()), 1235);
    // }

    // #[test]
    // fn test_real_part2_2() {
    //     assert_eq!(part_2_2(get_real_input()), 3051);
    // }
}
