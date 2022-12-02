// pub type GenOut = ???;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> GenOut {}

#[aoc(day2, part1)]
pub fn part_1(input: &GenOut) -> isize {}

#[aoc(day2, part2)]
pub fn part_2(input: &GenOut) -> isize {}

mod test {

    use super::*;

    #[allow(dead_code)]
    fn get_test_input() -> GenOut {
        let input: &str = "";
        input_generator(input)
    }

    #[allow(dead_code)]
    fn get_real_input() -> GenOut {
        input_generator(include_str!("../input/2022/day2.txt"))
    }

    #[test]
    fn test_example_part1() {
        let input = get_test_input();
        assert_eq!(part_1(&input), todo!());
    }

    #[test]
    fn test_example_part2() {
        let input = get_test_input();
        assert_eq!(part_2(&input), todo!());
    }

    #[test]
    fn test_real_part1() {
        let input = get_real_input();
        assert_eq!(part_1(&input), todo!());
    }

    #[test]
    fn test_real_part2() {
        let input = get_real_input();
        assert_eq!(part_2(&input), todo!());
    }
}
