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
            self.sub_dirs.push(contents[1].to_string());
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

    pub fn is_leaf(self) -> bool {
        return self.sub_dirs.is_empty();
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
                        let cwd = history.last().unwrap();
                        let mut dir = Directory::new(cwd.clone());
                        while let Some(ls_output) = output_lines_it.peek() {
                            if ls_output.starts_with('$') {
                                break;
                            }
                            dir.append_from_ls(String::from_str(ls_output).unwrap());
                            output_lines_it.next();
                        }
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

    pub fn directory_sizes(&self) -> HashMap<String, usize> {
        let cwd = &self.nodes[&String::from_str("/").unwrap()];
        let mut dir_sizes = HashMap::new();

        self.add_directory_size(&cwd, &mut dir_sizes);

        dir_sizes
    }

    pub fn root(&self) -> Directory {
        return self.nodes[&String::from_str("/").unwrap()].clone();
    }

    fn add_directory_size(&self, dir: &Directory, mut sizes: &mut HashMap<String, usize>) {
        println!("add_directory_size: {} => {:p}", dir.name.clone(), sizes);
        let mut size: usize = 0;
        dir.sub_dirs.clone().into_iter().for_each(|dir_name| {
            let sub_dir = self.nodes.get(&dir_name).unwrap();
            self.add_directory_size(sub_dir, &mut sizes);
            size += sizes.get(&dir_name).unwrap();
        });
        size += dir.files_size();
        sizes.insert(dir.name.clone(), size);
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
        println!(
            "start add_directory_size: {} => len {}",
            dir.name.clone(),
            self.size_map.len()
        );
        let mut size: usize = 0;
        dir.sub_dirs.clone().into_iter().for_each(|dir_name| {
            let sub_dir = fs.nodes.get(&dir_name).unwrap();
            self.add_directory_size(fs, sub_dir);
            size += self.size_map.get(&dir_name).unwrap();
        });
        size += dir.files_size();
        self.size_map.insert(dir.name.clone(), size);
        println!(
            "end add_directory_size: {} => len {}",
            dir.name.clone(),
            self.size_map.len()
        );
    }
}

fn sum_sizes_at_most(terminal_output: &String) -> String {
    let fs = FileSystem::from_terminal(terminal_output.clone());
    // println!("Nodes:");
    // println!("{:?}", fs);
    //let dir_sizes = fs.directory_sizes();
    let dir_sizes = DirectorySizes::new(&fs);
    // println!("Dir sizes:");
    // println!("{:?}", dir_sizes);
    println!(
        "num nodes: {} num sizes: {}",
        fs.nodes.len(),
        dir_sizes.size_map.len()
    );
    dir_sizes
        .size_map
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum::<usize>()
        .to_string()
}

fn main() {
    let dry_run = true;
    puzzle::Puzzle::new(2022, 7, puzzle::Id::PartOne).solve(sum_sizes_at_most, dry_run);
    //puzzle::Puzzle::new(2022, 6, puzzle::Id::PartTwo).solve(start_of_frame_location, dry_run);
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
