use std::collections::{HashMap, LinkedList};

pub fn process_part_1(input: &str) -> String {
    let mut folder_size: HashMap<String, u32> = HashMap::new();
    let mut path: LinkedList<&str> = LinkedList::new();

    for window in input
        .split("\n$ ")
        .into_iter()
        .collect::<Vec<&str>>()
        .windows(2)
    {
        let first_command = window[0];
        let second_command = window[1];

        if first_command.starts_with("cd ..") {
            path.pop_back().unwrap();
            continue;
        }
        if !second_command.starts_with("ls") || !first_command.starts_with("cd ") {
            continue;
        }

        let dir = first_command.split_whitespace().last().unwrap();
        path.push_back(dir);

        let path_as_a_string = get_path_as_a_string(&path);

        folder_size.insert(path_as_a_string, 0);

        for line in second_command.lines() {
            if line.starts_with("ls") || line.starts_with("dir") {
                continue;
            }

            let size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let mut temp_path = String::new();
            path.iter().for_each(|f| {
                temp_path.push_str(&format!("/{f}"));
                *folder_size.get_mut(temp_path.as_str()).unwrap() += size;
            });
        }
    }

    folder_size
        .values()
        .fold(0, |mut acc, v| {
            if v.to_owned() <= 100000 {
                acc += v;
            }
            acc
        })
        .to_string()
}

fn get_path_as_a_string(list: &LinkedList<&str>) -> String {
    let mut temp_path = String::new();
    list.iter().for_each(|f| {
        temp_path.push_str(&format!("/{f}"));
    });
    temp_path
}

pub fn process_part_2(input: &str) -> String {
    let mut folder_size: HashMap<String, u32> = HashMap::new();
    let mut path: LinkedList<&str> = LinkedList::new();

    for window in input
        .split("\n$ ")
        .into_iter()
        .collect::<Vec<&str>>()
        .windows(2)
    {
        let first_command = window[0];
        let second_command = window[1];

        if first_command.starts_with("cd ..") {
            path.pop_back().unwrap();
            continue;
        }
        if !second_command.starts_with("ls") || !first_command.contains("cd ") {
            continue;
        }

        let dir = first_command.split_whitespace().last().unwrap();
        path.push_back(dir);

        let path_as_a_string = get_path_as_a_string(&path);

        folder_size.insert(path_as_a_string, 0);

        for line in second_command.lines() {
            if line.starts_with("ls") || line.starts_with("dir") {
                continue;
            }

            let size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let mut temp_path = String::new();
            path.iter().for_each(|f| {
                temp_path.push_str(&format!("/{f}"));
                *folder_size.get_mut(temp_path.as_str()).unwrap() += size;
            });
        }
    }

    let total_space = 70000000;
    let used_space = folder_size.get("//").unwrap();
    let free_space = total_space - used_space;
    let needed_space = 30000000 - free_space;

    let mut values = folder_size.values().collect::<Vec<&u32>>();
    values.sort();

    let answer = values.iter().find(|v| v >= &&&needed_space).unwrap();

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn process_part_1_test() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "95437");
    }

    #[test]
    fn process_part_2_test() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "24933642");
    }
}
