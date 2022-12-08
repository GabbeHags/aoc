use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
enum FileSystemObj {
    File(usize),
    Dir(HashMap<String, FileSystemObj>),
}

#[derive(Debug)]
struct FileSystem {
    content: HashMap<PathBuf, FileSystemObj>,
    root: FileSystemObj,
    cd_path: PathBuf,
    cd: FileSystemObj,
}

impl FileSystem {
    fn new() -> Self {
        let content = HashMap::new();
        let root = FileSystemObj::Dir(HashMap::new());
        let cd_path = PathBuf::from("/");
        let cd = root.clone();
        Self {
            content,
            root,
            cd_path,
            cd,
        }
    }
    fn execute(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::cd(arg) => {
                match arg.as_str() {
                    ".." => self.cd_path = self.cd_path.parent().unwrap().to_path_buf(),
                    "/" => self.cd_path = PathBuf::from("/"),
                    dir => self.cd_path.push(dir),
                }
                if self.content.get(&self.cd_path).is_none() {
                    self.content
                        .insert(self.cd_path.clone(), FileSystemObj::Dir(HashMap::new()));
                } else {
                    todo!()
                }
            }
            Cmd::ls(objs) => {
                for name in objs {
                    match name.split_once(' ') {
                        Some((_type, name)) => {
                            if _type == "dir" {
                                let mut path = self.cd_path.to_path_buf();
                                path.push(name);

                                if self.content.get(&self.cd_path).is_none() {
                                    self.content.insert(
                                        self.cd_path.clone(),
                                        FileSystemObj::Dir(HashMap::new()),
                                    );
                                } else {
                                    todo!()
                                }
                            } else {
                                let mut path = self.cd_path.to_path_buf();
                                path.push(name);
                                self.content.insert(
                                    path,
                                    FileSystemObj::File(_type.parse::<usize>().unwrap()),
                                );
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Cmd {
    cd(String),
    ls(Vec<String>),
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> u32 {
    let mut lines_owned = input.lines().peekable();
    lines_owned.next();
    let lines = lines_owned.by_ref();
    let mut cmds = vec![];
    while let Some(line) = lines.next() {
        dbg!(&line);
        if let Some(cmd) = line.strip_prefix("$ ") {
            dbg!(&cmd);
            if cmd.starts_with("cd") {
                let mut arg = String::new();
                cmd.bytes().skip(3).for_each(|c| arg.push(c as char));
                cmds.push(Cmd::cd(arg));
            } else if cmd.starts_with("ls") {
                let mut v = vec![];
                while let Some(l) = lines.peek() {
                    dbg!(l);
                    if !l.starts_with('$') {
                        v.push(lines.next().unwrap().to_string())
                    } else {
                        break;
                    }
                }
                cmds.push(Cmd::ls(v));
            }
        }
    }

    let mut file_sys = FileSystem::new();
    for cmd in cmds {
        println!("------------START-------------");
        dbg!(&cmd);
        dbg!(&file_sys);
        println!("----------AFTER CMD-----------");
        file_sys.execute(cmd);
        dbg!(&file_sys);
        println!("------------END-------------");
    }

    !0
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> u32 {
    !0
}

mod test {
    #![allow(unreachable_code, unused_imports)]
    use super::*;

    #[allow(dead_code)]
    fn get_example_input<'a>() -> &'a str {
        "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
        .split_once('\n')
        .unwrap()
        .1
    }

    #[allow(dead_code)]
    fn get_real_input<'a>() -> &'a str {
        include_str!("../input/2022/day7.txt")
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part_1(get_example_input()), 95437);
    }

    // #[test]
    // fn test_example_part2() {
    //     assert_eq!(part_2(get_example_input()), 0);
    // }

    // #[test]
    // fn test_real_part1() {
    //     assert_eq!(part_1(get_real_input()), 0);
    // }

    // #[test]
    // fn test_real_part2() {
    //     assert_eq!(part_2(get_real_input()), 0);
    // }
}
