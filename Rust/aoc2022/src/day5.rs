fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

// #[aoc_generator(day5)]
// pub fn gen(input: &str) -> (Vec<Vec<Option<_>>>, Vec<[u8; 3]>) {

// }

#[aoc(day5, part1)]
pub fn part_1(input: &str) -> String {
    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();

    let mut stacks_tmp: Vec<Vec<Option<_>>> = vec![];
    let mut instructions: Vec<[u8; 3]> = vec![];

    for stack_str in stacks_str.lines() {
        let mut lvl = vec![];
        for (_, c) in stack_str.chars().enumerate().filter(|(j, _)| j % 4 == 1) {
            if c == ' ' {
                lvl.push(None);
            } else {
                lvl.push(Some(c))
            }
        }
        stacks_tmp.push(lvl);
    }
    stacks_tmp.pop();

    let mut stacks_tmp = transpose(stacks_tmp);
    let mut stacks = vec![];

    for row in &mut stacks_tmp {
        row.reverse();
        let mut r = vec![];
        for c in row.iter().filter(|c| c.is_some()) {
            r.push(c.unwrap());
        }
        stacks.push(r);
    }

    for line in instructions_str.lines() {
        let mut line_bytes = line.bytes().skip(5);
        instructions.push([
            line_bytes
                .by_ref()
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
            line_bytes
                .by_ref()
                .skip(5)
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
            line_bytes
                .skip(3)
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
        ]);
    }

    for [amount, from, to] in instructions {
        let mut pops = vec![];
        for _ in 0..amount {
            pops.push(stacks[(from - 1) as usize].pop().unwrap());
        }
        stacks[(to - 1) as usize].extend(pops);
    }

    let mut result = "".to_string();
    for row in stacks {
        if let Some(l) = row.last() {
            result.push(*l)
        }
    }
    result
}

#[aoc(day5, part2)]
pub fn part_2(input: &str) -> String {
    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();

    let mut stacks_tmp: Vec<Vec<Option<_>>> = vec![];
    let mut instructions: Vec<[u8; 3]> = vec![];

    for stack_str in stacks_str.lines() {
        let mut lvl = vec![];
        for (_, c) in stack_str.chars().enumerate().filter(|(j, _)| j % 4 == 1) {
            if c == ' ' {
                lvl.push(None);
            } else {
                lvl.push(Some(c))
            }
        }
        stacks_tmp.push(lvl);
    }
    stacks_tmp.pop();

    let mut stacks_tmp = transpose(stacks_tmp);
    let mut stacks = vec![];

    for row in &mut stacks_tmp {
        row.reverse();
        let mut r = vec![];
        for c in row.iter().filter(|c| c.is_some()) {
            r.push(c.unwrap());
        }
        stacks.push(r);
    }

    for line in instructions_str.lines() {
        let mut line_bytes = line.bytes().skip(5);
        instructions.push([
            line_bytes
                .by_ref()
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
            line_bytes
                .by_ref()
                .skip(5)
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
            line_bytes
                .skip(3)
                .take_while(|c| c.is_ascii_digit())
                .fold(0, |acc, d| acc * 10 + (d & 0b00001111)),
        ]);
    }

    for [amount, from, to] in instructions {
        let mut pops = vec![];
        for _ in 0..amount {
            pops.push(stacks[(from - 1) as usize].pop().unwrap());
        }
        pops.reverse();
        stacks[(to - 1) as usize].extend(pops);
    }

    let mut result = "".to_string();
    for row in stacks {
        if let Some(l) = row.last() {
            result.push(*l)
        }
    }
    result
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
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()).as_str(), "CMZ");
    }

    #[test]
    fn test_example_part2_() {
        assert_eq!(part_2(get_example_input()).as_str(), "MCD");
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()).as_str(), "VJSFHWGFT");
    }

    // #[test]
    // fn test_real_part2() {
    //     assert_eq!(part_2(get_real_input()).as_str(), "");
    // }
}
