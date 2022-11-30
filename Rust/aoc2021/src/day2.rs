pub struct Command {
    cmd: Cmds,
    num: i32,
}

pub enum Cmds {
    Forward,
    Up,
    Down,
}

impl Command {
    fn parse(s: &str) -> Command {
        const COMMAND_FORWARD: &str = "forward";
        const COMMAND_UP: &str = "up";
        const COMMAND_DOWN: &str = "down";

        let (cmd, num) = s.split_once(" ").unwrap();
        let cmd = match cmd {
            COMMAND_FORWARD => Cmds::Forward,
            COMMAND_UP => Cmds::Up,
            COMMAND_DOWN => Cmds::Down,
            _ => panic!(),
        };
        Command {
            cmd,
            num: num.parse().unwrap(),
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input.lines().map(|x| Command::parse(x)).collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn part_1(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in input {
        match command.cmd {
            Cmds::Forward => horizontal += command.num,
            Cmds::Up => depth -= command.num,
            Cmds::Down => depth += command.num,
        };
    }
    horizontal * depth
}

#[aoc(day2, part2)]
pub fn part_2(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in input {
        match command.cmd {
            Cmds::Forward => {
                horizontal += command.num;
                depth += aim * command.num;
            }
            Cmds::Up => aim -= command.num,
            Cmds::Down => aim += command.num,
        };
    }
    horizontal * depth
}
