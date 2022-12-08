use std::{collections::HashMap, path::{PathBuf}};

fn build_data(input: &str) -> HashMap<PathBuf, Vec<String>>{
    let mut lines = input.lines().peekable();
    lines.next();
    let mut _fs: HashMap<PathBuf, Vec<String>> = HashMap::new();
    let mut path = PathBuf::from("/");
    while let Some(line) = lines.next() {
        if let Some(cmd) = line.strip_prefix("$ ") {
            match cmd {
                "ls" => {
                    let mut v = vec![];
                    while let Some(l) = lines.peek() {
                        if !l.starts_with('$') {
                            v.push(lines.next().unwrap().to_string())
                        } else {
                            break;
                        }
                    }
                    if let Some(dir) = _fs.get_mut(&path) {
                        dir.extend(v);
                    } else {
                        _fs.insert(path.clone(), v);
                    }
                }
                _ => {
                    let mut arg = String::new();
                    cmd.bytes().skip(3).for_each(|c| arg.push(c as char));
                    match arg.as_str() {
                        ".." => path = path.parent().unwrap().into(),
                        "/" => path = "/".into(),
                        _ => path = path.join(arg),
                    }
                }
            }
        }
    }
    _fs
}

fn read_file_sizes(input: &str) -> HashMap<PathBuf, usize>{
    let _fs = build_data(input);
    let mut current_depth = 0;
    let mut stack = vec![("/".to_string(), 0)];
    let mut fs_size: HashMap<PathBuf, usize> = HashMap::new();
    let mut path = PathBuf::new();
    while !stack.is_empty() {
        let (name, depth) = stack.pop().unwrap();
        while depth < current_depth {
            let dir_size = *fs_size.get(&path).unwrap();
            path = path.parent().unwrap().to_path_buf();
            if let Some(size) = fs_size.get_mut(&path) {
                *size += dir_size;
            }
            current_depth -= 1;
        }
        path = path.join(name);
        current_depth += 1;
        let dir = _fs.get(&path).unwrap();
        for content in dir {
            if let Some(dir) = content.strip_prefix("dir ") {
                stack.push({
                    let mut folder = String::new();
                    dir.bytes().for_each(|c| folder.push(c as char));
                    if fs_size.get(&path.join(&folder)).is_none() {
                        fs_size.insert(path.join(&folder), 0);
                    }
                    (folder, current_depth)
                });
            } else {
                let n = content.split_once(' ').unwrap().0.parse::<usize>().unwrap();
                if let Some(size) = fs_size.get_mut(&path) {
                    *size += n;
                } else {
                    fs_size.insert(path.clone(), n);
                }
            }
        }
    }
    while current_depth > 1 {
        let dir_size = *fs_size.get(&path).unwrap();
        path = path.parent().unwrap().to_path_buf();
        if let Some(size) = fs_size.get_mut(&path) {
            *size += dir_size;
        } else {
            unreachable!()
        }
        current_depth -= 1;
    }

    fs_size
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> usize {
    let folder_sizes = read_file_sizes(input);
    folder_sizes
        .into_values()
        .filter(|val| *val <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> usize {
    let folder_sizes = read_file_sizes(input);

    const TOTAL_SPACE: usize  = 70_000_000;
    const UPDATE_SIZE: usize = 30_000_000;
    let space_left = TOTAL_SPACE - folder_sizes.get(&PathBuf::from("/")).unwrap();
    let space_needed = UPDATE_SIZE - space_left;

    folder_sizes
        .into_values()
        .filter(|val| *val >= space_needed)
        .min().unwrap()
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

    #[test]
    fn test_example_part2() {
        assert_eq!(part_2(get_example_input()), 24933642);
    }

    #[test]
    fn test_real_part1() {
        assert_eq!(part_1(get_real_input()), 1644735);
    }

    #[test]
    fn test_real_part2() {
        assert_eq!(part_2(get_real_input()), 1300850);
    }
}
