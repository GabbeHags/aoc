use aoc_runner_derive::{aoc, aoc_generator};

trait ParseInstruction
where
    Self: Sized,
{
    fn parse(data: &str) -> Option<(&str, Self)>;
}

trait DoAction {
    fn action(&self) -> usize;
}

#[derive(Debug)]
struct Mul {
    x: usize,
    y: usize,
}

impl DoAction for Mul {
    fn action(&self) -> usize {
        self.x * self.y
    }
}

impl ParseInstruction for Mul {
    fn parse(data: &str) -> Option<(&str, Self)> {
        if !data.starts_with("mul(") {
            return None;
        }
        let data = &data[4..];
        let mut count = 0;
        let mut x: usize = 0;
        for b in data.as_bytes().iter() {
            count += 1;
            if *b == b',' {
                count -= 1;
                break;
            } else if count > 3 {
                return None;
            } else if b.is_ascii_digit() {
                x *= 10;
                x += b.wrapping_sub(b'0') as usize;
            } else {
                return None;
            }
        }
        let data = &data[count..];
        if !data.starts_with(',') {
            return None;
        }
        let data = &data[1..];
        let mut count = 0;
        let mut y: usize = 0;
        for b in data.as_bytes().iter() {
            count += 1;
            if *b == b')' {
                count -= 1;
                break;
            } else if count > 3 {
                return None;
            } else if b.is_ascii_digit() {
                y *= 10;
                y += b.wrapping_sub(b'0') as usize;
            } else {
                return None;
            }
        }
        let data = &data[count..];
        if !data.starts_with(')') {
            return None;
        }
        Some((&data[1..], Self { x, y }))
    }
}

#[derive(Debug)]
struct Do;

impl ParseInstruction for Do {
    fn parse(data: &str) -> Option<(&str, Self)> {
        if !data.starts_with("do()") {
            return None;
        }
        Some((&data["do()".len()..], Self))
    }
}

#[derive(Debug)]
struct Dont;

impl ParseInstruction for Dont {
    fn parse(data: &str) -> Option<(&str, Self)> {
        if !data.starts_with("don't()") {
            return None;
        }
        Some((&data["don't()".len()..], Self))
    }
}

#[derive(Debug)]
enum Instruction {
    Mul(Mul),
    Do(Do),
    Dont(Dont),
}

impl Instruction {
    fn parse_instructions(data: &str) -> Vec<Self> {
        let mut data = data;
        let mut res = vec![];
        loop {
            if let Some((d, ins)) = Mul::parse(data) {
                data = d;
                res.push(Instruction::Mul(ins));
            } else if let Some((d, ins)) = Do::parse(data) {
                data = d;
                res.push(Instruction::Do(ins));
            } else if let Some((d, ins)) = Dont::parse(data) {
                data = d;
                res.push(Instruction::Dont(ins));
            } else if data.len() > 1 {
                data = &data[1..];
            } else {
                break;
            }
        }
        res
    }

    fn action(&self) -> usize {
        match self {
            Instruction::Mul(mul) => mul.action(),
            _ => unimplemented!(),
        }
    }
}

type ParseOut = Vec<Instruction>;

#[aoc_generator(day3)]
fn parse(input: &str) -> ParseOut {
    let instructions = Instruction::parse_instructions(input);
    // dbg!(&instructions);
    // dbg!(&instructions);
    instructions
}

#[aoc(day3, part1)]
fn part1(input: &ParseOut) -> usize {
    input
        .iter()
        .filter(|ins| matches!(ins, Instruction::Mul(_)))
        .map(|ins| ins.action())
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &ParseOut) -> usize {
    let mut do_act = true;
    let mut sum = 0;
    for ins in input {
        match ins {
            Instruction::Mul(mul) => {
                if do_act {
                    sum += mul.action();
                }
            }
            Instruction::Do(_) => do_act = true,
            Instruction::Dont(_) => do_act = false,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_input() -> String {
        std::fs::read_to_string("./input/2024/day3.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    fn example_input_part1() -> String {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()
    }
    fn example_input_part2() -> String {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(&parse(&real_input())), 159892596);
    }

    #[test]
    fn part2_real() {
        assert_eq!(part2(&parse(&real_input())), 92626942);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&example_input_part1())), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&example_input_part2())), 48);
    }
}
