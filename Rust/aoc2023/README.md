# Advent of Code 2023 in Rust
My attempt at Advent of Code 2023.

This project uses [`cargo-aoc`][cargo-aoc] to compile and run each day.

## Days

Time is my best time with my best solutions. 
The time include both the generation and solving of each part.

See [Bench Times](bench-times) to see the benched time.

All solutions are measured on a `Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz`.

| Advent of Code Links |         Solutions         |             Time Part 1             |              Time Part 2           |
|:--------------------:|:-------------------------:|:-----------------------------------:|:----------------------------------:|
| [Day 1][day01]       | [Solution](./src/day1.rs)  | [19.991 µs](bench-times/day1.txt)  | [26.508 µs](bench-times/day1.txt)  |
| [Day 2][day02]       | [Solution](./src/day2.rs)  | [11.239 µs](bench-times/day2.txt)  | [11.405 µs](bench-times/day2.txt)  |
| [Day 3][day03]       | [Solution](./src/day3.rs)  | [221.32 µs](bench-times/day3.txt)  | [207.05 µs](bench-times/day3.txt)  |
| [Day 4][day04]       | [Solution](./src/day4.rs)  | [108.95 µs](bench-times/day4.txt)  | [162.01 µs](bench-times/day4.txt)  |
| [Day 5][day05]       | [Solution](./src/day5.rs)  | [32.345 µs](bench-times/day5.txt)  | [24.2759 s](bench-times/day5.txt)  |
| [Day 6][day06]       | [Solution](./src/day6.rs)  | [263.90 ns](bench-times/day6.txt)  | [106.62 ns](bench-times/day6.txt)  |
<!-- | [Day 7][day07]       | [Solution](./src/day7.rs)  | [???????us](bench-times/day7.txt)  | [???????us](bench-times/day7.txt)  | -->
<!-- | [Day 8][day08]       | [Solution](./src/day8.rs)  | [???????us](bench-times/day8.txt)  | [???????us](bench-times/day8.txt)  | -->
<!-- | [Day 9][day09]       | [Solution](./src/day9.rs)  | [???????us](bench-times/day9.txt)  | [???????us](bench-times/day9.txt)  | -->
<!-- | [Day 10][day10]      | [Solution](./src/day10.rs) | [???????us](bench-times/day10.txt) | [???????us](bench-times/day10.txt) | -->
<!-- | [Day 11][day11]      | [Solution](./src/day11.rs) | [???????us](bench-times/day11.txt) | [???????us](bench-times/day11.txt) | -->
<!-- | [Day 12][day12]      | [Solution](./src/day12.rs) | [???????us](bench-times/day12.txt) | [???????us](bench-times/day12.txt) | -->
<!-- | [Day 13][day13]      | [Solution](./src/day13.rs) | [???????us](bench-times/day13.txt) | [???????us](bench-times/day13.txt) | -->
<!-- | [Day 14][day14]      | [Solution](./src/day14.rs) | [???????us](bench-times/day14.txt) | [???????us](bench-times/day14.txt) | -->
<!-- | [Day 15][day15]      | [Solution](./src/day15.rs) | [???????us](bench-times/day15.txt) | [???????us](bench-times/day15.txt) | -->
<!-- | [Day 16][day16]      | [Solution](./src/day16.rs) | [???????us](bench-times/day16.txt) | [???????us](bench-times/day16.txt) | -->
<!-- | [Day 17][day17]      | [Solution](./src/day17.rs) | [???????us](bench-times/day17.txt) | [???????us](bench-times/day17.txt) | -->
<!-- | [Day 18][day18]      | [Solution](./src/day18.rs) | [???????us](bench-times/day18.txt) | [???????us](bench-times/day18.txt) | -->
<!-- | [Day 19][day19]      | [Solution](./src/day19.rs) | [???????us](bench-times/day19.txt) | [???????us](bench-times/day19.txt) | -->
<!-- | [Day 20][day20]      | [Solution](./src/day20.rs) | [???????us](bench-times/day20.txt) | [???????us](bench-times/day20.txt) | -->
<!-- | [Day 21][day21]      | [Solution](./src/day21.rs) | [???????us](bench-times/day21.txt) | [???????us](bench-times/day21.txt) | -->
<!-- | [Day 22][day22]      | [Solution](./src/day22.rs) | [???????us](bench-times/day22.txt) | [???????us](bench-times/day22.txt) | -->
<!-- | [Day 23][day23]      | [Solution](./src/day23.rs) | [???????us](bench-times/day23.txt) | [???????us](bench-times/day23.txt) | -->
<!-- | [Day 24][day24]      | [Solution](./src/day24.rs) | [???????us](bench-times/day24.txt) | [???????us](bench-times/day24.txt) | -->
<!-- | [Day 25][day25]      | [Solution](./src/day25.rs) | [???????us](bench-times/day15.txt) | [???????us](bench-times/day15.txt) | -->

[day01]: https://adventofcode.com/2023/day/1
[day02]: https://adventofcode.com/2023/day/2
[day03]: https://adventofcode.com/2023/day/3
[day04]: https://adventofcode.com/2023/day/4
[day05]: https://adventofcode.com/2023/day/5
[day06]: https://adventofcode.com/2023/day/6
[day07]: https://adventofcode.com/2023/day/7
[day08]: https://adventofcode.com/2023/day/8
[day09]: https://adventofcode.com/2023/day/9
[day10]: https://adventofcode.com/2023/day/10
[day11]: https://adventofcode.com/2023/day/11
[day12]: https://adventofcode.com/2023/day/12
[day13]: https://adventofcode.com/2023/day/13
[day14]: https://adventofcode.com/2023/day/14
[day15]: https://adventofcode.com/2023/day/15
[day16]: https://adventofcode.com/2023/day/16
[day17]: https://adventofcode.com/2023/day/17
[day18]: https://adventofcode.com/2023/day/18
[day19]: https://adventofcode.com/2023/day/19
[day20]: https://adventofcode.com/2023/day/20
[day21]: https://adventofcode.com/2023/day/21
[day22]: https://adventofcode.com/2023/day/22
[day23]: https://adventofcode.com/2023/day/23
[day24]: https://adventofcode.com/2023/day/24
[day25]: https://adventofcode.com/2023/day/25

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

### Get the benchmarks of the latest solution.
```bash
cargo aoc bench -g
```

## Solutions written with
* rustc 1.74.0
* cargo-aoc 0.3.5


---

[aoc]: https://adventofcode.com/
[rust]: https://rust-lang.org
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
[aoc-runner]: https://github.com/gobanos/aoc-runner