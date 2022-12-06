fn generate_stacks(input: &str) -> Vec<Vec<u8>> {
    let mut lines = input.lines().rev();
    let stacks_num = lines
        .next()
        .unwrap()
        .bytes()
        .enumerate()
        .filter(|(j, _)| j % 4 == 1)
        .count();

    let mut stacks = vec![vec![]; stacks_num];
    for (i, (_, c)) in lines
        .flat_map(|line| {
            line.bytes()
                .enumerate()
                .filter(|(i, c)| i % 4 == 1 && (c.is_ascii_uppercase() || *c == b' '))
        })
        .enumerate()
    {
        if c != b' ' {
            stacks[i % stacks_num].push(c);
        }
    }
    stacks
}

fn generate_instruction(line: &str) -> [u32; 3] {
    let mut line_bytes = line.bytes().skip(5);
    [
        line_bytes
            .by_ref()
            .take_while(|c| c.is_ascii_digit())
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111) as u32),
        line_bytes
            .by_ref()
            .skip(5)
            .take_while(|c| c.is_ascii_digit())
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111) as u32),
        line_bytes
            .skip(3)
            .take_while(|c| c.is_ascii_digit())
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111) as u32),
    ]
}

fn get_top_of_stacks(stacks: &[Vec<u8>]) -> String {
    let mut result = "".to_string();
    for row in stacks {
        if let Some(l) = row.last() {
            result.push(*l as char);
        }
    }
    result
}

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> String {
    let (stack_lines, instruction_lines) = input.split_once("\n\n").unwrap();
    let mut stacks = generate_stacks(stack_lines);
    let mut pops = Vec::with_capacity(stacks[0].len());
    for [amount, from, to] in instruction_lines.lines().map(generate_instruction) {
        let stack_len = stacks[(from - 1) as usize].len();
        let drain = stacks[(from - 1) as usize].drain(
            (stack_len - amount as usize)
                ..stack_len,
        ).rev();
        pops.extend(drain);
        stacks[(to - 1) as usize].extend(&pops);
        pops.clear();
    }

    get_top_of_stacks(&stacks)
}

#[aoc(day5, part2)]
pub fn part_2(input: &str) -> String {
    let (stack_lines, instruction_lines) = input.split_once("\n\n").unwrap();
    let mut stacks = generate_stacks(stack_lines);
    let mut pops = Vec::with_capacity(stacks[0].len());
    for [amount, from, to] in instruction_lines.lines().map(generate_instruction) {
        let stack_len = stacks[(from - 1) as usize].len();
        let drain = stacks[(from - 1) as usize].drain(
            (stack_len - amount as usize)
                ..stack_len,
        );
        pops.extend(drain);
        stacks[(to - 1) as usize].extend(&pops);
        pops.clear();
    }

    get_top_of_stacks(&stacks)
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_real_input<'a>() -> &'a str {
        include_str!("../input/2022/day5.txt")
    }

    #[test]
    fn test_example_part1_2() {
        assert_eq!(part_1(get_example_input()).as_str(), "CMZ");
    }

    #[test]
    fn test_example_part2_2() {
        assert_eq!(part_2(get_example_input()).as_str(), "MCD");
    }

    #[test]
    fn test_real_part1_2() {
        assert_eq!(part_1(get_real_input()).as_str(), "VJSFHWGFT");
    }

    #[test]
    fn test_real_part2_2() {
        assert_eq!(part_2(get_real_input()).as_str(), "LCTQFBVZV");
    }
}
