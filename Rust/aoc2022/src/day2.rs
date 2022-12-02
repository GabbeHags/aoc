pub type GenOut = Vec<[RPS; 2]>;
pub type GenOut2 = Vec<(RPS, Round)>;

#[derive(Debug, Clone, Copy)]
pub enum RPS {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Round {
    Win,
    Draw,
    Lose,
}

impl Round {
    fn get_score(&self) -> Score {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn from_letter(letter: char) -> Self {
        match letter {
            'X' => Self::Win,
            'Y' => Self::Draw,
            'Z' => Self::Lose,
            _ => unreachable!(),
        }
    }
}

type Score = u32;

impl RPS {
    fn from_letter(letter: char) -> Self {
        match letter {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissor,
            _ => unreachable!(),
        }
    }


    fn get_score(&self) -> Score {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn win_over(&self, other: &Self) -> Round {
        match (self, other) {
            (Self::Rock, Self::Scissor) => Round::Win,
            (Self::Paper, Self::Rock) => Round::Win,
            (Self::Scissor, Self::Paper) => Round::Win,
            (Self::Rock, Self::Rock) => Round::Draw,
            (Self::Paper, Self::Paper) => Round::Draw,
            (Self::Scissor, Self::Scissor) => Round::Draw,
            _ => Round::Lose,
        }
    }

    fn get_rsp_to_result_in(&self, end_goal: Round) -> RPS {
        if end_goal == Round::Draw {
            self.clone()
        } else {
            match (self, end_goal) {
                (Self::Rock, Round::Win) => Self::Scissor,
                (Self::Rock, Round::Lose) => Self::Paper,
                (Self::Paper, Round::Win) => Self::Rock,
                (Self::Paper, Round::Lose) => Self::Scissor,
                (Self::Scissor, Round::Win) => Self::Paper,
                (Self::Scissor, Round::Lose) => Self::Rock,
                _ => unreachable!(),
            }
        }
    }
}

#[aoc_generator(day2, part1)]
pub fn input_generator(input: &str) -> GenOut {
    input
        .lines()
        .map(|s| {
            let mut chars = s.chars();
            let opp = RPS::from_letter(chars.next().unwrap());
            chars.next();
            let my = RPS::from_letter(chars.next().unwrap());
            [opp, my]
        })
        .collect()
}
#[aoc_generator(day2, part2)]
pub fn input_generator2(input: &str) -> GenOut2 {
    input
        .lines()
        .map(|s| {
            let mut chars = s.chars();
            let opp = RPS::from_letter(chars.next().unwrap());
            chars.next();
            let result_should_be = Round::from_letter(chars.next().unwrap());
            (opp, result_should_be)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part_1(input: &GenOut) -> u32 {
    // dbg!(&input);
    let mut total_score = 0;
    for round in input {
        let opp = &round[0];
        let me = &round[1];
        let round_score = me.win_over(&opp).get_score();
        let hand_score = me.get_score();
        total_score += round_score + hand_score;
    }
    total_score
}

#[aoc(day2, part2)]
pub fn part_2(input: &GenOut2) -> u32 {
    // dbg!(&input);
    let mut total_score = 0;
    for (opp, result) in input {
        let me = opp.get_rsp_to_result_in(*result);
        let round_score = me.win_over(&opp).get_score();
        let hand_score = me.get_score();
        total_score += round_score + hand_score;
    }
    total_score
}

mod test {
    #![allow(unreachable_code)]
    use super::*;

    fn get_example_input() -> &'static str {
        "A Y
B X
C Z"
    }

    #[allow(dead_code)]
    fn get_real_input() -> &'static str {
        include_str!("../input/2022/day2.txt")
    }

    #[allow(dead_code)]
    fn gen_example_input() -> GenOut {
        input_generator(get_example_input())
    }
    #[allow(dead_code)]
    fn gen_example_input2() -> GenOut2 {
        input_generator2(get_example_input())
    }

    #[allow(dead_code)]
    fn gen_real_input() -> GenOut {
        input_generator(get_real_input())
    }
    #[allow(dead_code)]
    fn gen_real_input2() -> GenOut2 {
        input_generator2(get_real_input())
    }

    #[test]
    fn test_example_part1() {
        let input = gen_example_input();
        assert_eq!(part_1(&input), 15);
    }

    #[test]
    fn test_example_part2() {
        let input = gen_example_input2();
        assert_eq!(part_2(&input), 12);
    }

    #[test]
    fn test_real_part1() {
        let input = gen_real_input();
        assert_eq!(part_1(&input), 12740);
    }

    #[test]
    fn test_real_part2() {
        let input = gen_real_input2();
        assert_eq!(part_2(&input), 11980);
    }
}
