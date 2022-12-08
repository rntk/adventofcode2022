use std::{fs, fmt};
use std::str::FromStr;
use crate::days::day2::ParseError;

use regex::Regex;

pub fn letters(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut crates = Crates::new();
    for (i, s) in strings.enumerate() {
        let ts = s.trim();
        if ts == "" {
            continue
        }
        if ts.to_lowercase().starts_with("move") {
            if let Err(e) = crates.move_crate(s) {
                return format!("Invalid row: {} - {} - {}", i, s, e)
            }
        } else if ts.starts_with("1") {
            continue
        } else {
            if  let Err(e) = crates.add_crates(s) {
                return format!("Invalid row: {} - {} - {}", i, s, e)
            }
        }
    }

    return crates.top()
}

pub fn letters_order(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut crates = Crates::new();
    for (i, s) in strings.enumerate() {
        let ts = s.trim();
        if ts == "" {
            continue
        }
        if ts.to_lowercase().starts_with("move") {
            if let Err(e) = crates.move_crate_order(s) {
                return format!("Invalid row: {} - {} - {}", i, s, e)
            }
        } else if ts.starts_with("1") {
            continue
        } else {
            if  let Err(e) = crates.add_crates(s) {
                return format!("Invalid row: {} - {} - {}", i, s, e)
            }
        }
    }

    return crates.top()
}

#[derive(Debug)]
pub struct Crates {
    crates: Vec<Vec<char>>
}

impl Crates {
    pub fn new() -> Crates {
        Crates{crates: vec![]}
    }

    pub fn add_crates(&mut self, s: &str) -> Result<(), ParseError> {
        let mut column = 0;
        let step = 4;
        for i in (0..s.len()).step_by(step) {
            if self.crates.len() <= column {
                self.crates.push(vec![])
            }
            let mut end = i + step;
            if end >= s.len() {
                end -= 1
            }
            let chrs = s[i..end].trim();
            if chrs.trim() != "" {
                if !chrs.starts_with("[") || !chrs.ends_with("]") {
                    return Err(ParseError { msg: format!("Invalid letter: {} - '{}'", i, chrs) })
                }
                let chrs_v:Vec<char> = chrs.chars().collect();
                self.crates[column].push(chrs_v[1]);
            }
            column += 1;
        }

        return Ok(())
    }

    pub fn move_crate(&mut self, s: &str) -> Result<(), MoveError> {
        let mv: Movement = s.parse()?;
        let from = (mv.from - 1) as usize;
        if self.crates.len() <= from {
            return Err(MoveError{msg: format!("Invalid from: {}", mv.from)})
        }
        if self.crates[from].len() < mv.number as usize {
            return Err(MoveError{msg: format!("Invalid number: {}", mv.number)})
        }
        let to = (mv.to - 1) as usize;
        if self.crates.len() <= to {
            return Err(MoveError{msg: format!("Invalid to: {}", mv.to)})
        }
        for _ in 0..mv.number {
            let ch = self.crates[from][0];
            self.crates[from].remove(0);
            self.crates[to].insert(0, ch);
        }

        return Ok(())
    }

    pub fn move_crate_order(&mut self, s: &str) -> Result<(), MoveError> {
        let mv: Movement = s.parse()?;
        let from = (mv.from - 1) as usize;
        if self.crates.len() <= from {
            return Err(MoveError{msg: format!("Invalid from: {}", mv.from)})
        }
        if self.crates[from].len() < mv.number as usize {
            return Err(MoveError{msg: format!("Invalid number: {}", mv.number)})
        }
        let to = (mv.to - 1) as usize;
        if self.crates.len() <= to {
            return Err(MoveError{msg: format!("Invalid to: {}", mv.to)})
        }
        for i in (0..mv.number).rev() {
            let ch = self.crates[from][i as usize];
            self.crates[from].remove(i as usize);
            self.crates[to].insert(0, ch);
        }

        return Ok(())
    }

    pub fn top(&self) -> String {
        let mut s: Vec<String> = vec![];
        for v in self.crates.iter() {
            s.push(v[0].to_string())
        }

        return s.join("").to_string()
    }
}

pub struct MoveError {
    pub msg: String
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<ParseError> for MoveError {
    fn from(e: ParseError) -> Self {
        MoveError {
            msg: format!("Crate move error: {}", e),
        }
    }
}

#[derive(Debug)]
struct Movement {
    pub from: i32,
    pub number: i32,
    pub to: i32
}

impl FromStr for Movement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let move_re = Regex::new(r"^\s*move\s+([0-9]+)\s+from\s+([0-9]+)\s+to\s+([0-9]+)\s*$").unwrap();
        let mut inds: [i32;3] = [0, 0, 0];
        for cap in move_re.captures_iter(s) {
            for i in 1..=3 {
                let indx: i32 = cap[i].parse()?;
                inds[i - 1] = indx
            }
        }

        return Ok(Movement{
            from: inds[1],
            number: inds[0],
            to: inds[2]
        })
    }
}

