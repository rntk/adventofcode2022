use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use crate::days::day2::ParseError;

pub fn size(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let dirs = match build_fs(input) {
        Ok(drs) => drs,
        Err(e) => return format!("Fs error: {}", e)
    };

    let (_, sum) = sum_dirs(&dirs, 0);

    return sum.to_string()
}

pub fn size_deleted(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let dirs = match build_fs(input) {
        Ok(drs) => drs,
        Err(e) => return format!("Fs error: {}", e)
    };

    let (current_size, _) = sum_dirs(&dirs, 0);
    let total_storage = 70000000;
    let required = 30000000;
    let to_free = required - (total_storage - current_size);
    let(_, deleted_size) = sizes(&dirs, 0, to_free as i64);

    return match deleted_size {
        Some(s) => s.to_string(),
        None => "Not found".to_string()
    }
}

fn build_fs(strings: String) -> Result<Vec<Directory>, ParseError> {
    let mut dirs = vec![Directory::new("/", None)];
    let mut current_dir = 0;
    for (i, s) in strings.split("\n").enumerate() {
        let st = s.trim();
        if st == "" {
            continue
        }
        if st.starts_with("$ ") {
            let cmd: Command = match st.parse() {
                Ok(c) => c,
                Err(e) => return Err(ParseError{msg: format!("Line {}. {}. {}", i, s, e)})
            };
            match cmd {
                Command::CD(dir) => {
                    if dir == ".." {
                        current_dir = match dirs[current_dir].parent {
                            Some(cd) => cd,
                            None => return Err(ParseError{msg: format!("No parent dir. line: {}. {}", i, s)})
                        }
                    } else if dir == "/" {
                        current_dir = 0
                    } else {
                        current_dir = match dirs[current_dir].dirs.get(&dir) {
                            Some(d) => *d,
                            None => return Err(ParseError{msg: format!("No found dir. line: {}. {}", i, s)})
                        }
                    }
                },
                Command::LS => {
                    continue
                },
            }
        } else if st.starts_with("dir ") {
            let name = st.trim_start_matches("dir").trim();
            let d = Directory::new(
                name,
                Some(current_dir)
            );
            dirs.push(d);
            let ln = dirs.len();
            dirs[current_dir].dirs.insert(name.to_string(), ln - 1);
        } else {
            let f: File = match s.parse() {
                Ok(fl) => fl,
                Err(e) => return Err(ParseError{msg: format!("File parsing: Line: {}. {}. {}", i, i, e)})
            };
            dirs[current_dir].files.push(f)
        }
    }

    return Ok(dirs)
}

enum Command {
    LS,
    CD(String),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ts = s.trim();
        if !ts.starts_with("$") {
            return Err(ParseError { msg: format!("Invalid command: {}", s) })
        }
        ts = ts.trim_start_matches("$").trim();
        if ts.starts_with("cd ") {
            return Ok(Command::CD(ts[3..].to_string()))
        }
        if ts.starts_with("ls") {
            return Ok(Command::LS)
        }

        return Err(ParseError { msg: format!("Unknown command: {}", s) })
    }
}

struct File {
    size: u64,
    //name: String
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl: Vec<&str> = s.split(" ").collect();
        if spl.len() < 2 {
            return Err(ParseError { msg: format!("Invalid file: {}", s) })
        }
        let size: u64 = match spl[0].parse() {
            Ok(s) => s,
            Err(e) => return Err(ParseError { msg: format!("Failed to parse size: {}. {}", s, e) })
        };

        return Ok(File {
            //name: spl[1..].join(" ").trim().to_string(),
            size: size
        })
    }
}

struct Directory {
    //pub name: String,
    pub files: Vec<File>,
    pub dirs: HashMap<String, usize>,
    pub parent: Option<usize>
}

impl Directory {
    pub fn new(_name: &str, parent: Option<usize>) -> Directory {
        Directory{
            //name: name.to_string(),
            files: vec![],
            dirs: HashMap::new(),
            parent: parent
        }
    }
}

impl Directory {
    pub fn files_size(&self) -> u64 {
        let mut size: u64 = 0;
        for f in &self.files {
            size += f.size
        }

        return size
    }
}

fn sum_dirs(dirs: &Vec<Directory>, current_dir: usize) -> (u64, u64) {
    let max_size = 100000;
    let files_size = dirs[current_dir].files_size();
    if dirs[current_dir].dirs.len() == 0 {
        return (files_size, 0)
    }
    let mut sizes: Vec<(u64, u64)> = vec![];
    for (_, d ) in &dirs[current_dir].dirs {
        sizes.push(sum_dirs(&dirs, *d))
    }
    let mut dirs_sum = 0;
    let mut directory_size = files_size;
    for (subd_size, subd_sum) in sizes {
        if subd_size <= max_size {
            dirs_sum += subd_size
        }
        dirs_sum += subd_sum;
        directory_size += subd_size
    }

    return (directory_size, dirs_sum)
}

fn sizes(dirs: &Vec<Directory>, current_dir: usize, to_free: i64) -> (u64, Option<u64>) {
    let files_size = dirs[current_dir].files_size();
    if dirs[current_dir].dirs.len() == 0 {
        if to_free - files_size as i64 > 0 {
            return (files_size, None)
        }

        return (files_size, Some(files_size))
    }
    let mut dir_size = files_size;
    let mut sub_mins: Vec<u64> = vec![];
    for (_, d ) in &dirs[current_dir].dirs {
        let (subd_size, subd_min) = sizes(dirs, *d, to_free);
        dir_size += subd_size;
        if let Some(min) = subd_min {
            sub_mins.push(min)
        }
    }
    if to_free - dir_size as i64 <= 0 {
        sub_mins.push(dir_size);
    }
    if sub_mins.len() > 0 {
        sub_mins.sort();

        return (dir_size, Some(sub_mins[0]))
    }

    return (dir_size, None)
}