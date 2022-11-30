fn is_increasing(l: Option<i32>, r: i32) -> i32 {
    if let Some(l) = l {
        if l < r {
            return 1;
        }
    }
    0
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
fn part_1(input: &[i32]) -> i32 {
    let mut prev: Option<i32> = None;
    let mut count: i32 = 0;
    for curr in input {
        count += is_increasing(prev, *curr);
        prev = Some(*curr)
    }
    count
}

#[aoc(day1, part2)]
pub fn part_2(input: &[i32]) -> i32 {
    let mut prev: Option<i32> = None;
    let mut count: i32 = 0;
    let mut curr: i32;
    for i in 0..input.len() - 2 {
        curr = input[i] + input[i + 1] + input[i + 2];
        count += is_increasing(prev, curr);

        prev = Some(curr);
    }
    count
}
