use std::collections::BTreeSet;

pub type Elf = u32;
pub type GenOutVec = Vec<Elf>;
pub type GenOutSet = BTreeSet<Elf>;

#[aoc_generator(day1, part1, vec)]
#[aoc_generator(day1, part2, vec)]
pub fn input_generator_vec(input: &str) -> GenOutVec {
    input
        .split("\n\n")
        .map(|str| {
            str.lines()
                .map(|calories| calories.parse::<Elf>().unwrap())
                .sum()
        })
        .collect()
}

#[aoc_generator(day1, part1, set)]
#[aoc_generator(day1, part2, set)]
pub fn input_generator_set(input: &str) -> GenOutSet {
    let mut set: BTreeSet<Elf> = BTreeSet::new();
    for elf in input.split("\n\n") {
        set.insert(
            elf.lines()
                .map(|calories| calories.parse::<Elf>().unwrap())
                .sum(),
        );
    }
    set
}

#[aoc(day1, part1, vec)]
pub fn part_1_vec(input: &GenOutVec) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part1, set)]
pub fn part_1_set(input: &GenOutSet) -> u32 {
    *input.into_iter().next_back().unwrap()
}

#[aoc(day1, part2, vec)]
pub fn part_2_vec(input: &GenOutVec) -> u32 {
    let mut top_3 = [0, 0, 0];
    for elf_calories in input {
        if top_3[0] <= *elf_calories {
            top_3[2] = top_3[1];
            top_3[1] = top_3[0];
            top_3[0] = *elf_calories;
        } else if top_3[1] <= *elf_calories {
            top_3[2] = top_3[1];
            top_3[1] = *elf_calories;
        } else if top_3[2] <= *elf_calories {
            top_3[2] = *elf_calories;
        }
    }
    top_3.iter().sum()
}

#[aoc(day1, part2, set)]
pub fn part_2_set(input: &GenOutSet) -> u32 {
    let mut rev_input = input.into_iter().rev();
    rev_input.next().unwrap() + rev_input.next().unwrap() + rev_input.next().unwrap()
}

mod test {

    use super::*;

    fn get_example_input() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    #[allow(dead_code)]
    fn get_real_input() -> &'static str {
        include_str!("../input/2022/day1.txt")
    }

    #[allow(dead_code)]
    fn gen_example_input_vec() -> GenOutVec {
        input_generator_vec(get_example_input())
    }

    #[allow(dead_code)]
    fn gen_example_input_set() -> GenOutSet {
        input_generator_set(get_example_input())
    }

    #[allow(dead_code)]
    fn gen_real_input_vec() -> GenOutVec {
        input_generator_vec(get_real_input())
    }

    #[allow(dead_code)]
    fn gen_real_input_set() -> GenOutSet {
        input_generator_set(get_real_input())
    }

    #[test]
    fn test_example_part1() {
        let input = gen_example_input_vec();
        assert_eq!(part_1_vec(&input), 24000);
        let input = gen_example_input_set();
        assert_eq!(part_1_set(&input), 24000);
    }

    #[test]
    fn test_example_part2() {
        let input = gen_example_input_vec();
        assert_eq!(part_2_vec(&input), 45000);
        let input = gen_example_input_set();
        assert_eq!(part_2_set(&input), 45000);
    }

    #[test]
    fn test_real_part1() {
        let input = gen_real_input_vec();
        assert_eq!(part_1_vec(&input), 68467);
        let input = gen_real_input_set();
        assert_eq!(part_1_set(&input), 68467);
    }

    #[test]
    fn test_real_part2() {
        let input = gen_real_input_vec();
        assert_eq!(part_2_vec(&input), 203420);
        let input = gen_real_input_set();
        assert_eq!(part_2_set(&input), 203420);
    }
}
