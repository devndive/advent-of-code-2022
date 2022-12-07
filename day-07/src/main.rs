#[cfg(test)]
mod tests {
    use crate::{is_command, is_directory, is_file};

    #[test]
    fn is_command_should_work() {
        assert_eq!(is_command("$ cd /"), true);
        assert_eq!(is_command("$ ls"), true);
        assert_eq!(is_command("dir a"), false);
        assert_eq!(is_command("14848514 b.txt"), false);
        assert_eq!(is_command("8504156 c.txt"), false);
        assert_eq!(is_command("dir d"), false);
    }

    #[test]
    fn is_directory_should_work() {
        assert_eq!(is_directory("$ cd /"), false);
        assert_eq!(is_directory("$ ls"), false);
        assert_eq!(is_directory("dir a"), true);
        assert_eq!(is_directory("14848514 b.txt"), false);
        assert_eq!(is_directory("8504156 c.txt"), false);
        assert_eq!(is_directory("dir d"), true);
    }

    #[test]
    fn is_file_should_work() {
        assert_eq!(is_file("$ cd /"), false);
        assert_eq!(is_file("$ ls"), false);
        assert_eq!(is_file("dir a"), false);
        assert_eq!(is_file("14848514 b.txt"), true);
        assert_eq!(is_file("8504156 c.txt"), true);
        assert_eq!(is_file("29116 f"), true);
        assert_eq!(is_file("dir d"), false);
    }
}

use std::{
    collections::{HashMap},
    fs,
};

fn is_command(input: &str) -> bool {
    if input.starts_with("$") {
        return true;
    }

    false
}

fn is_directory(input: &str) -> bool {
    if input.starts_with("dir") {
        return true;
    }

    false
}

fn is_file(input: &str) -> bool {
    let parts = input.split_whitespace().collect::<Vec<&str>>();

    if parts.len() == 2 && parts[0].parse::<u32>().is_ok() {
        return true;
    }

    false
}

enum RowType {
    Command,
    Directory,
    File,
}

fn get_row_type(input: &str) -> RowType {
    if is_command(input) {
        return RowType::Command;
    } else if is_directory(input) {
        return RowType::Directory;
    } else if is_file(input) {
        return RowType::File;
    } else {
        panic!("Unknown row type: {}", input);
    }
}

struct File {
    _name: String,
    size: u32,
}

impl File {
    // create new file from command "12345 test.fiz"
    pub fn new(input: &str) -> Self {
        let parts = input
            .split_whitespace()
            .map(|part| String::from(part))
            .collect::<Vec<String>>();

        Self {
            _name: parts[1].clone(),
            size: parts[0].parse::<u32>().unwrap(),
        }
    }
}

struct Directory {
    files: Vec<File>,
    directories: Vec<String>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            files: vec![],
            directories: vec![],
        }
    }

    pub fn size(&self) -> u32 {
        self.files.iter().fold(0, |acc, f| acc + f.size)
    }
}

fn total_size_rec(directories: &HashMap<String, Directory>, directory_name: &String) -> u32 {
    let dir = directories.get(directory_name).unwrap();

    let mut total_size = dir.size();

    if dir.directories.len() > 0 {
        total_size += dir.directories.iter().fold(0, |acc, sub_dir| {
            acc + total_size_rec(&directories, sub_dir)
        });
    }

    return total_size;
}

fn main() {
    let input = fs::read_to_string("./src/puzzle_input").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();

    let mut directories: HashMap<String, Directory> = HashMap::new();
    // The current directory holds the complete current path and always ends with a '/'
    // examples:
    // - /aabdf/ddde/
    // - /
    // - /a
    let mut current_directory: String = String::from("");

    for line in lines {
        match get_row_type(line) {
            RowType::Command => {
                // println!("Processing command");
                // the only command that requires to take action is 'cd'
                if line.contains("cd") {
                    let parts = line.split_whitespace().collect::<Vec<&str>>();
                    let directory_name = parts[2];

                    // For now we just assume that the only possible arguments for changing a directory are
                    // .. (go up one) or the folder we want to switch to.
                    if directory_name == ".." {
                        // remove the last char '/'
                        current_directory.pop();
                        // remove all chars till we find the next '/'
                        while current_directory.chars().last().unwrap() != '/' {
                            current_directory.pop(); // removes last directory name
                        }
                    } else {
                        if directory_name == "/" {
                            current_directory = String::from("/");
                        } else {
                            current_directory = format!("{}{}/", current_directory, directory_name);
                        }

                        if !directories.contains_key(&current_directory) {
                            directories
                                .insert(current_directory.clone(), Directory::new());
                        }
                    }
                }
            }
            RowType::Directory => {
                // println!("Processing command");
                let parts = line
                    .split_whitespace()
                    .map(|part| String::from(part))
                    .collect::<Vec<String>>();

                let cur_dir = directories.get_mut(&current_directory).unwrap();

                cur_dir
                    .directories
                    .push(format!("{}{}/", &current_directory, parts[1]));
            }
            RowType::File => {
                // println!("Processing file");
                let cur_dir = directories.get_mut(&current_directory).unwrap();

                cur_dir.files.push(File::new(line));
            }
        }
    }

    println!(
        "Part 1: {}",
        directories.iter().fold(0, |acc, (directory_name, _)| {
            let total_dir_size = total_size_rec(&directories, directory_name);

            if total_dir_size < 100000 {
                return acc + total_dir_size;
            }

            acc
        })
    );

    let needed = 30000000;
    let current_used = total_size_rec(&directories, &String::from("/"));
    let current_free = 70000000 - current_used;

    // part two
    let mut sizes: Vec<u32> = vec![];
    for (directory_name, _) in directories.iter() {
        let total_dir_size = total_size_rec(&directories, directory_name);
        sizes.push(total_dir_size);
    }

    sizes.sort();

    let min_size = sizes.iter().find(|s| current_free + **s >= needed);
    println!("Part 2: {}", min_size.unwrap());
}
