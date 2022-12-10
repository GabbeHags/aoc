#[derive(Debug)]
enum Ins {
    Addx(isize),
    Noop,
}

impl Ins {
    fn get_cycles(&self) -> usize {
        match self {
            Ins::Addx(_) => 2,
            Ins::Noop => 1,
        }
    }

    fn execute(&self, x: &mut isize) {
        match self {
            Ins::Addx(n) => *x += n,
            Ins::Noop => (),
        }
    }
}

const NOOP: &str = "noop";
const ADDX: &str = "addx";

impl From<&str> for Ins {
    fn from(s: &str) -> Self {
        if s.starts_with(NOOP) {
            Self::Noop
        } else {
            let (_, num) = s.split_once(' ').unwrap();
            Self::Addx(num.parse().unwrap())
        }
    }
}

#[aoc(day10, part1)]
pub fn part_1(input: &str) -> isize {
    let mut total_cycles = 0;
    let mut x = 1;
    let mut sum = 0;
    for line in input.lines() {
        let ins = Ins::from(line);
        for _cycle in 0..ins.get_cycles() {
            total_cycles += 1;
            if total_cycles % 40 == 20 {
                sum += total_cycles * x;
            }
        }
        ins.execute(&mut x);
    }

    sum
}

#[aoc(day10, part2)]
pub fn part_2(input: &str) -> String {
    let mut total_cycles = 0;
    let mut x = 1;
    let mut screen = String::new();
    for line in input.lines() {
        let ins = Ins::from(line);
        for _cycle in 0..ins.get_cycles() {
            total_cycles += 1;
            if (x-1..=x+1).contains(&((total_cycles % 40)-1)) && (x > 0 || x < 40) {
                screen += "#";
            } else {
                screen += ".";
            }
            if total_cycles % 40 == 0 {
                screen += "\n";
            }
        }
        ins.execute(&mut x);
    }
    // println!("{screen}");
    // FECZELHE
    screen
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input_2<'a>() -> &'a str {
        "
noop
addx 3
addx -5
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_real_input() -> &'static str {
        include_str!("../input/2022/day10.txt").trim_end()
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 13140);
    }
    #[test]
    fn test_example_part1_2() {
        assert_eq!(part_1(get_example_input_2()), 0);
    }

    #[test]
    fn test_example_part2() {
        let correct = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part_2(get_example_input()), correct.to_string());
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 12540);
    }

    // #[test]
    // fn test_real_part2() {
    //     assert_eq!(part_2(get_real_input()), 0);
    // }
}
