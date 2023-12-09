use std::{cmp::Ordering, collections::HashSet, hint::unreachable_unchecked, ops::Range};

use aoc_runner_derive::aoc;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapName {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl<T: AsRef<str>> From<T> for MapName {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => panic!("Unknown"),
        }
    }
}

#[derive(Debug)]
struct Map {
    _src: MapName,
    dst: MapName,
    ranges: Vec<(Range<usize>, Range<usize>)>,
}
impl Map {
    fn get_dst(&self, in_src: usize) -> usize {
        if let Ok((src_range, dst_range)) = self
            .ranges
            .binary_search_by(|(src, _dst)| {
                if src.start > in_src {
                    Ordering::Greater
                } else if src.end <= in_src {
                    Ordering::Less
                } else {
                    debug_assert!(src.contains(&in_src));
                    Ordering::Equal
                }
            })
            .map(|index| &self.ranges[index])
        {
            dst_range.start + (in_src - src_range.start)
        } else {
            in_src
        }
    }
    fn _merge_ranges(&mut self) {
        dbg!(&self.ranges.len());
        loop {
            let mut remove = Vec::new();
            let mut found = HashSet::new();
            self.ranges.sort_by(|a, b| a.0.start.cmp(&b.0.start));

            // todo!();
            for (src, dst) in &self.ranges {
                for (filter_src, filter_dst) in &self.ranges {
                    if src == filter_src && dst == filter_dst {
                        continue;
                    } else if src.end == filter_src.start && dst.end == filter_dst.start {
                        remove.push((src.clone(), dst.clone()));
                        found.insert((src.start..filter_src.end, dst.start..filter_dst.end));
                    } else if src.start == filter_src.end && dst.start == filter_dst.end {
                        remove.push((src.clone(), dst.clone()));
                        found.insert((filter_src.start..src.end, filter_dst.start..dst.end));
                    } else if src.end == filter_src.start && dst.start == filter_dst.end {
                        remove.push((src.clone(), dst.clone()));
                        found.insert((src.start..filter_src.end, filter_dst.start..dst.end));
                    } else if src.start == filter_src.end && dst.end == filter_dst.start {
                        remove.push((src.clone(), dst.clone()));
                        found.insert((filter_src.start..src.end, dst.start..filter_dst.end));
                    } else if (src.contains(&filter_src.start) || filter_src.contains(&src.start))
                        && (dst.contains(&filter_dst.start) || filter_dst.contains(&dst.start))
                    {
                        remove.push((src.clone(), dst.clone()));
                        found.insert((
                            src.start.min(filter_src.start)..src.end.max(filter_src.end),
                            dst.start.min(filter_dst.start)..dst.end.max(filter_dst.end),
                        ));
                    }
                }
            }
            if !found.is_empty() {
                for (remove_src, remove_dst) in remove {
                    self.ranges
                        .retain(|(src, dst)| !(*src == remove_src && *dst == remove_dst))
                }
                self.ranges.extend(found);
            } else {
                dbg!(&self.ranges.len());
                break;
            }
        }
    }
}
type Maps = SmallVec<[Map; 7]>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Maps,
}

impl Almanac {
    fn get_correspondent(&self, src_index: usize, src: MapName) -> (MapName, usize) {
        // let map = self.maps.iter().find(|map| map.dst == src).unwrap();
        let map: &Map = &self.maps[match src {
            MapName::Seed => unsafe { unreachable_unchecked() },
            MapName::Soil => 0,
            MapName::Fertilizer => 1,
            MapName::Water => 2,
            MapName::Light => 3,
            MapName::Temperature => 4,
            MapName::Humidity => 5,
            MapName::Location => 6,
        }];
        (map.dst, map.get_dst(src_index))
    }

    fn get_map_from_src(&self, src: MapName) -> &Map {
        &self.maps[match src {
            MapName::Seed => 0,
            MapName::Soil => 1,
            MapName::Fertilizer => 2,
            MapName::Water => 3,
            MapName::Light => 4,
            MapName::Temperature => 5,
            MapName::Humidity => 6,
            MapName::Location => unsafe { unreachable_unchecked() },
        }]
    }

    fn get_to_correspondent_from_seed(
        &self,
        in_seed: usize,
        end_correspondent: MapName,
    ) -> (MapName, usize) {
        let (mut src, mut src_index) = self.get_correspondent(in_seed, MapName::Soil);
        loop {
            if src == end_correspondent {
                return (src, src_index);
            }
            (src, src_index) = self.get_correspondent(src_index, self.get_map_from_src(src).dst);
        }
    }
}

fn parse(input: &str) -> Almanac {
    let mut iter = input.split("\n\n");
    let seeds = iter.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    let maps: Maps = iter
        .map(|map| {
            let mut iter = map.lines();
            let (src, dst) = iter.next().unwrap().split_once("-to-").unwrap();
            let dst = dst.strip_suffix(" map:").unwrap();
            let mut ranges: Vec<(Range<usize>, Range<usize>)> = iter
                .map(|range| {
                    let mut iter = range.split_ascii_whitespace();
                    let range_dst: usize = iter.next().unwrap().parse().unwrap();
                    let range_src: usize = iter.next().unwrap().parse().unwrap();
                    let range_len: usize = iter.next().unwrap().parse().unwrap();
                    (
                        range_src..(range_src + range_len),
                        range_dst..(range_dst + range_len),
                    )
                })
                .collect();
            ranges.sort_by(|a, b| a.0.start.cmp(&b.0.start));
            Map {
                _src: src.into(),
                dst: dst.into(),
                ranges,
            }
        })
        .collect();

    debug_assert!(!maps.spilled());

    Almanac { seeds, maps }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    let parsed = parse(input);

    parsed
        .seeds
        .iter()
        .map(|seed| {
            parsed
                .get_to_correspondent_from_seed(*seed, MapName::Location)
                .1
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    let parsed = parse(input);

    parsed
        .seeds
        .par_chunks_exact(2)
        .flat_map(|range| range[0]..(range[0] + range[1]))
        .map(|seed| {
            parsed
                .get_to_correspondent_from_seed(seed, MapName::Location)
                .1
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
        .trim()
    }

    fn real_input() -> String {
        std::fs::read_to_string("./input/2023/day5.txt")
            .unwrap()
            .trim()
            .to_string()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(example_input()), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(example_input()), 46);
    }

    #[test]
    fn part1_real() {
        assert_eq!(part1(&real_input()), 178159714);
    }

    // #[test]
    // fn part2_real() {
    //     assert_eq!(part2(&real_input()), 100165128);
    // }
}
