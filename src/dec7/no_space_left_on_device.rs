use std::iter::Skip;
use std::str::{Lines, Split};
use regex::Regex;
use crate::get_data_as_string;

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
}

impl Dir {
    fn get_size(&self) -> Vec<(String, usize, bool)> {
        let mut dirs_and_size: Vec<(String, usize, bool)> = Vec::new();
        let mut top_dir_size = 0;
        for dir in &self.dirs {
            let subdirs = dir.get_size();
            let mut dir_size = 0;
            for (_, size, counted) in subdirs.iter() {
                dir_size += if !counted {size} else {&0};
            }
            top_dir_size += dir_size;
            let mut subdirs_counted:Vec<(String, usize, bool)> = subdirs.iter()
                .map(|(name,size,_)|(name.to_string(),*size,true)).collect();
            dirs_and_size.append(&mut subdirs_counted);
        }
        if self.files.is_empty() {
            return dirs_and_size;
        }
        top_dir_size += self.files.iter()
            .map(|file|file.get_size())
            .fold(0, |acc, file| acc + file);
        dirs_and_size.push((self.name.clone(), top_dir_size, false));
        dirs_and_size
    }
    fn print(self, number_of_tabs:usize) {
        let tabs = "\t".to_string().repeat(number_of_tabs);
        println!("{}{}:", tabs, self.name);
        for file in self.files {
            file.print(number_of_tabs+1);
        }
        for dir in self.dirs {
            dir.print(number_of_tabs + 1);
        }
    }
    fn new(name: String) -> Dir {
        Dir {
            name,
            dirs: vec![],
            files: vec![],
        }
    }
    fn ls(&mut self, unparsed_string: Skip<Lines>, where_to_insert: Vec<String>) {
        let mut insert_dir = self;
        for dir in where_to_insert {
            match insert_dir.clone().does_dir_exist(dir.clone()) {
                None => {
                    panic!("Dir {} not in {:?}", dir, insert_dir)
                }
                Some(index) => {
                    insert_dir = insert_dir.dirs.get_mut(index).unwrap();
                }
            }
        }
        for line in unparsed_string {
            if Regex::new(r"dir\s\w+").unwrap().is_match(line) {
                let (_, dir_name) = line.split_at(4);
                insert_dir.dirs.push(Dir::new(dir_name.to_string()));
            }
            else if Regex::new(r"\d+\s.+").unwrap().is_match(line) {
                let mut parts = line.split_whitespace();
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let name = parts.next().unwrap();
                insert_dir.files.push(File::new(name.to_string(), size));
            }
            else {
                panic!("Unknown line in ls instruction: {}", line);
            }
        }
    }
    fn does_dir_exist(self, name: String) -> Option<usize> {
        for (index,dir) in self.dirs.iter().enumerate() {
            match dir.name == name {
                true => {return Some(index)}
                false => {}
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn get_size(&self) -> usize {
        self.size
    }

    fn print(self, number_of_tabs:usize) {
        let tabs = "\t".to_string().repeat(number_of_tabs);
        println!("{}{} {}",tabs,self.name,self.size);
    }

    fn new(name: String, size: usize) -> File {
        File {
            name,
            size,
        }
    }
}

#[derive(Clone, Debug)]
struct FileSystem {
    path: Vec<String>,
    root: Dir,
}

impl FileSystem {

    fn get_size(&self, threshold: usize) -> usize {
        let fs = self.root.get_size();
        println!("{:?}", fs);
        fs.iter()
            .fold(0, |acc, (_, size,_)
            | if size <= &threshold {acc + size} else {acc})
    }

    fn print(self) {
        self.root.print(0);
    }

    fn create_fs(mut self, instrs: Skip<Split<&str>>) -> Self {
        let cd_regex = Regex::new(r"^cd\s.+\n").unwrap();
        let back_regex = Regex::new(r"cd\s\.\.\n").unwrap();
        let ls_regex = Regex::new(r"^ls(\n(dir|\d+)\s\w+)+").unwrap();
        for instr in instrs {
            if cd_regex.is_match(&instr) {
                if back_regex.is_match(&instr) {
                    if self.path.pop().is_none() {
                        panic!("cd instr tried to move to nonexistant folder");
                    }
                }
                else {
                    let without_newline = instr.lines().next().unwrap();
                    if without_newline.len() < 4 {
                        println!("instr: {} \n ::{}", instr ,without_newline);
                    }
                    let (_, dir_name) = without_newline.split_at(3);
                    self.path.push(dir_name.to_string());
                }
            }
            else if ls_regex.is_match(&instr) {
                let lines = instr.lines();
                let without_ls = lines.skip(1);
                self.root.ls(without_ls, self.path.clone());
            }
            else {
                panic!("Unknown instruction: {}", instr);
            }
        }
        self
    }
}

pub fn no_space_left_on_device(is_example: bool) -> usize {
    let unparsed_string = get_data_as_string(is_example, "dec7");
    let list = FileSystem {
        path: vec![],
        root: Dir {
            name: "root".to_string(),
            dirs: vec![Dir::new("/".to_string())],
            files: vec![],
        }
    };
    let instrs = unparsed_string.split("$ ").skip(1);
    let fs = FileSystem::create_fs(list, instrs);
    fs.clone().print();
    fs.get_size(58381165)
}

