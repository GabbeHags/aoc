# Advent of Code 2022 in Rust
My attempt at Advent of Code 2022.

This project uses [`aoc-runner`][aoc-runner] and [`cargo-aoc`][cargo-aoc]

### Private leaderboard
* [Leaderboard](https://adventofcode.com/2022/leaderboard/private/view/642677)

## Days

|             Advent of Code Links               |               Solutions                    |
|:----------------------------------------------:|:------------------------------------------:|
| [Day 1](https://adventofcode.com/2022/day/1)   | [Code](./src/day1.rs) &nbsp;&nbsp; `Part 1: 931.39 ns`, `Part 2: 1.0157 us` |
| [Day 2](https://adventofcode.com/2022/day/2)   | [Code](./src/day2.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` |
<!-- | [Day 3](https://adventofcode.com/2022/day/3)   | [Code](./src/day3.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 4](https://adventofcode.com/2022/day/4)   | [Code](./src/day4.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 5](https://adventofcode.com/2022/day/5)   | [Code](./src/day5.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 6](https://adventofcode.com/2022/day/6)   | [Code](./src/day6.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 7](https://adventofcode.com/2022/day/7)   | [Code](./src/day7.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 8](https://adventofcode.com/2022/day/8)   | [Code](./src/day8.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 9](https://adventofcode.com/2022/day/9)   | [Code](./src/day9.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 10](https://adventofcode.com/2022/day/10) | [Code](./src/day10.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 11](https://adventofcode.com/2022/day/11) | [Code](./src/day11.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 12](https://adventofcode.com/2022/day/12) | [Code](./src/day12.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 13](https://adventofcode.com/2022/day/13) | [Code](./src/day13.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 14](https://adventofcode.com/2022/day/14) | [Code](./src/day14.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 15](https://adventofcode.com/2022/day/15) | [Code](./src/day15.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 16](https://adventofcode.com/2022/day/16) | [Code](./src/day16.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 17](https://adventofcode.com/2022/day/17) | [Code](./src/day17.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 18](https://adventofcode.com/2022/day/18) | [Code](./src/day18.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 19](https://adventofcode.com/2022/day/19) | [Code](./src/day19.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 20](https://adventofcode.com/2022/day/20) | [Code](./src/day20.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 21](https://adventofcode.com/2022/day/21) | [Code](./src/day21.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 22](https://adventofcode.com/2022/day/22) | [Code](./src/day22.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 23](https://adventofcode.com/2022/day/23) | [Code](./src/day23.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 24](https://adventofcode.com/2022/day/24) | [Code](./src/day24.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->
<!-- | [Day 25](https://adventofcode.com/2022/day/25) | [Code](./src/day25.rs) &nbsp;&nbsp; `Part 1: `, `Part 2: ` | -->


## Usage

### Each `dayX.rs` should have this these functions.
```rust
#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> input_to_parts {
    todo!()
}

#[aoc(dayX, part1)]
pub fn part_1(input: input_to_parts) -> part1_answer {
    todo!()
}

#[aoc(dayX, part2)]
pub fn part_2(input: input_to_parts) -> part2_answer {
    todo!()
}
```

### Get the result of the latest solution.
```bash
cargo aoc
```


## Solutions written with
* rustc 1.65.0
* cargo-aoc 0.3.0


---

[aoc]: https://adventofcode.com/
[rust]: https://rust-lang.org
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
[aoc-runner]: https://github.com/gobanos/aoc-runner