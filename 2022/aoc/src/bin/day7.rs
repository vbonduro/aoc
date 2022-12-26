use aoc::puzzle;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    sub_dirs: Vec<String>,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name: name,
            sub_dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn append_from_ls(&mut self, ls_output: String) {
        let contents = ls_output.split_whitespace().collect::<Vec<&str>>();
        if contents[0] == "dir" {
            let subdir_path = format!("{}/{}", self.name, contents[1]);
            self.sub_dirs.push(subdir_path);
        } else {
            self.files.push(File {
                name: contents[1].to_string(),
                size: contents[0].parse().unwrap(),
            })
        }
    }

    pub fn files_size(&self) -> usize {
        return self.files.clone().into_iter().map(|file| file.size).sum();
    }
}

#[derive(Debug)]
struct FileSystem {
    nodes: HashMap<String, Directory>,
}

impl FileSystem {
    pub fn from_terminal(output: String) -> FileSystem {
        let mut fs = FileSystem {
            nodes: HashMap::new(),
        };
        let mut history = Vec::new();

        let mut output_lines_it = output.lines().into_iter().peekable();
        while let Some(output_line) = output_lines_it.next() {
            if output_line.starts_with('$') {
                let command_bits = output_line.split_whitespace().collect::<Vec<&str>>();
                match command_bits[1] {
                    "cd" => {
                        if command_bits[2] == ".." {
                            history.pop();
                        } else {
                            history.push(String::from_str(command_bits[2]).unwrap());
                        }
                    }
                    "ls" => {
                        let cwd = history.join("/");
                        let mut dir = Directory::new(cwd.clone());
                        while let Some(ls_output) = output_lines_it.peek() {
                            if ls_output.starts_with('$') {
                                break;
                            }
                            dir.append_from_ls(String::from_str(ls_output).unwrap());
                            output_lines_it.next();
                        }
                        println!("{}", cwd);
                        assert!(fs.nodes.insert(cwd.clone(), dir).is_none(), "{}", cwd);
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        }

        fs
    }

    pub fn root(&self) -> Directory {
        return self.nodes[&String::from_str("/").unwrap()].clone();
    }
}

struct DirectorySizes {
    size_map: HashMap<String, usize>,
}

impl DirectorySizes {
    pub fn new(fs: &FileSystem) -> Self {
        let mut sizes_map = DirectorySizes {
            size_map: HashMap::new(),
        };

        sizes_map.add_directory_size(fs, &fs.root());

        sizes_map
    }

    fn add_directory_size(&mut self, fs: &FileSystem, dir: &Directory) {
        let mut size: usize = 0;
        dir.sub_dirs.clone().into_iter().for_each(|dir_name| {
            let sub_dir = fs.nodes.get(&dir_name).unwrap();
            self.add_directory_size(fs, sub_dir);
            size += self.size_map.get(&dir_name).unwrap();
        });
        size += dir.files_size();
        self.size_map.insert(dir.name.clone(), size);
    }
}

fn sum_sizes_at_most(terminal_output: &String) -> String {
    let fs = FileSystem::from_terminal(terminal_output.clone());
    let dir_sizes = DirectorySizes::new(&fs);

    dir_sizes
        .size_map
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum::<usize>()
        .to_string()
}

fn size_of_dir_to_delete(terminal_output: &String) -> String {
    let fs = FileSystem::from_terminal(terminal_output.clone());
    let dir_sizes = DirectorySizes::new(&fs);

    let disk_space = 70000000;
    let unused_space_required = 30000000;

    let used_space = dir_sizes.size_map.get(&String::from("/")).unwrap();
    let unused_space = disk_space - used_space;
    let target_dir_size = unused_space_required - unused_space;

    dir_sizes
        .size_map
        .into_iter()
        .filter(|(_, size)| *size >= target_dir_size)
        .map(|(_, size)| size)
        .min()
        .unwrap()
        .to_string()
}

fn main() {
    let dry_run = false;
    //puzzle::Puzzle::new(2022, 7, puzzle::Id::PartOne).solve(sum_sizes_at_most, dry_run);
    puzzle::Puzzle::new(2022, 7, puzzle::Id::PartTwo).solve(size_of_dir_to_delete, dry_run);
}

mod tests {
    use crate::sum_sizes_at_most;
    use crate::FileSystem;
    #[test]
    fn test_sample_1() {
        let console_log = String::from(
            "$ cd /
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
7214296 k",
        );
        let fs = FileSystem::from_terminal(console_log.clone());
        println!("{:?}", fs);
        let sizes = fs.directory_sizes();
        println!("{:?}", sizes);
        assert_eq!(sum_sizes_at_most(&console_log), 95437.to_string());
    }
}
