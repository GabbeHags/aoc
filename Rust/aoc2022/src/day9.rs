use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn mov(&mut self, dir: &Dir, steps: isize) {
        match dir {
            Dir::U => self.y += steps,
            Dir::D => self.y -= steps,
            Dir::L => self.x -= steps,
            Dir::R => self.x += steps,
        }
    }

    fn mov_delta(&mut self, delta_x: isize, delta_y: isize) {
        self.x += delta_x;
        self.y += delta_y;
    }

    fn is_touching(&self, pos: &Pos) -> bool {
        let (delta_x, delta_y) = self.delta(pos);
        (-1..=1).contains(&delta_x) && (-1..=1).contains(&delta_y)
    }

    fn delta(&self, pos: &Pos) -> (isize, isize) {
        (self.x - pos.x, self.y - pos.y)
    }

    fn mov_towards(&mut self, pos: &Pos) {
        let (delta_x, delta_y) = pos.delta(self);
        self.mov_delta(delta_x.signum(), delta_y.signum());
    }
}


#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => unreachable!(),
        }
    }
}

#[aoc(day9, part1)]
pub fn part_1(input: &str) -> usize {

    const KNOTS: usize= 2;

    let mut last_visits = HashSet::<Pos>::new();
    let mut rope = (0..KNOTS).map(|_| Pos::new(0, 0)).collect::<Vec<_>>();
    for ins in input.lines() {
        let (dir, steps) = ins.split_once(' ').unwrap();
        let dir = Dir::from(dir);
        let steps = steps
            .bytes()
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111) as isize);
        for _ in 0..steps {
            rope[0].mov(&dir, 1);
            for knot_idx in 1..rope.len() {
                let following = rope[knot_idx - 1];
                if !following.is_touching(&rope[knot_idx]) {
                    rope[knot_idx].mov_towards(&following);
                }
            }
            last_visits.insert(rope[1]);
        }
    }
    last_visits.len()
}

#[aoc(day9, part2)]
pub fn part_2(input: &str) -> usize {

    const KNOTS: usize= 10;

    let mut last_visits = HashSet::<Pos>::new();
    let mut rope = (0..KNOTS).map(|_| Pos::new(0, 0)).collect::<Vec<_>>();
    for ins in input.lines() {
        let (dir, steps) = ins.split_once(' ').unwrap();
        let dir = Dir::from(dir);
        let steps = steps
            .bytes()
            .fold(0, |acc, d| acc * 10 + (d & 0b00001111) as isize);
        for _ in 0..steps {
            rope[0].mov(&dir, 1);
            for knot_idx in 1..rope.len() {
                let following = rope[knot_idx - 1];
                if !following.is_touching(&rope[knot_idx]) {
                    rope[knot_idx].mov_towards(&following);
                }
            }
            last_visits.insert(rope[9]);
        }
    }
    last_visits.len()
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_example_input_2<'a>() -> &'a str {
        "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_real_input<'a>() -> &'a str {
        include_str!("../input/2022/day9.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 13);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part_2(get_example_input()), 1);
    }

    #[test]
    fn test_example_part2_1() {
        assert_eq!(part_2(get_example_input_2()), 36);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 6563);
    }

    //     #[test]
    //     fn test_real_part2() {
    //         assert_eq!(part_2(get_real_input()), 0);
    //     }
}
