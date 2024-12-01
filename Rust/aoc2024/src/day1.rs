use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::zip;
use utils::parse::ParseOps;

type ParseOut = (Vec<usize>, Vec<usize>);

#[aoc_generator(day1)]
fn parse(input: &str) -> ParseOut {
    let lists = input.lines().map(|x| x.as_bytes()).map(|line| {
        let v1 = line[..(line.len() / 2)]
            .trim_ascii_end()
            .unsigned::<usize>();
        let v2 = line[(line.len() / 2)..]
            .trim_ascii_start()
            .unsigned::<usize>();
        (v1, v2)
    });

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for (n1, n2) in lists {
        vec1.push(n1);
        vec2.push(n2);
    }

    (vec1, vec2)
}

#[aoc(day1, part1)]
fn part1((in1, in2): &ParseOut) -> usize {
    let mut in1 = in1.clone();
    let mut in2 = in2.clone();
    in1.sort();
    in2.sort();

    zip(in1, in2).map(|(x1, x2)| x1.abs_diff(x2)).sum()
}

#[aoc(day1, part2)]
fn part2((in1, in2): &ParseOut) -> usize {
    let counts = utils::counting_set::CountingSet::from_iter(in2);

    in1.iter().map(|x| x * counts.get(&x).unwrap_or(0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn real_input() -> String {
        std::fs::read_to_string("./input/2024/day1.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    fn example_input() -> String {
        "3   4
4   3
2   5
1   3
3   9
3   3
"
        .to_string()
    }

    #[test]
    fn part1_real1() {
        assert!(part1(&parse(&real_input())) < 16081532);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(&example_input())), 11);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse("3   7")), 4);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&parse("7   7")), 0);
    }
    #[test]
    fn part1_example4() {
        assert_eq!(part1(&parse("41226   69190 ")), 27964);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&example_input())), 31);
    }
}
