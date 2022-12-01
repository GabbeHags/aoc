#[derive(Debug)]
pub struct Elf {
    foods: Vec<isize>,
}

impl Elf {
    pub fn get_total_calories(&self) -> isize {
        self.foods.iter().sum()
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|str| Elf {
            foods: str
                .split("\n")
                .map(|calories| calories.parse().unwrap())
                .collect(),
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part_1(input: &[Elf]) -> isize {
    input
        .iter()
        .map(|elf| elf.get_total_calories())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part_2(input: &[Elf]) -> isize {
    let mut top_3 = [0, 0, 0];
    for elf in input {
        let calories = elf.get_total_calories();
        if top_3[0] <= calories {
            top_3[2] = top_3[1];
            top_3[1] = top_3[0];
            top_3[0] = calories;
        } else if top_3[1] <= calories {
            top_3[2] = top_3[1];
            top_3[1] = calories;
        } else if top_3[2] <= calories {
            top_3[2] = calories;
        }
    }
    top_3.iter().sum()
}

mod test {

    use super::*;

    #[allow(dead_code)]
    fn get_test_input() -> Vec<Elf> {
        let input: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        input_generator(input)
    }

    #[test]
    fn test_part1() {
        let input = get_test_input();
        assert_eq!(part_1(&input), 24000);
    }
    #[test]
    fn test_part2() {
        let input = get_test_input();
        assert_eq!(part_2(&input), 45000);
    }
}
