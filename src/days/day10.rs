use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

use crate::days::day2::ParseError;

pub fn signals_sum(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut sum = 0;
    let mut check = HashSet::new();
    let checks = vec![20, 60, 100, 140,180, 220];
    for v in checks {
        check.insert(v);
    }
    let mut cycles = 0;
    let mut state = 1;
    for (line, s) in input.split("\n").enumerate() {
        if s.trim() == "" {
            continue
        }
        let instr: Instruction = match s.parse() {
            Ok(ins) => ins,
            Err(e) => return format!("Invalid: {} {}. {}", line, s, e)
        };
        for _ in 0..instr.cycles() {
            cycles += 1;
            if check.contains(&cycles) {
                sum += state * cycles
            }
        }
        if let Instruction::AddX(v) = instr {
            state += v
        }
    }

    return sum.to_string()
}

enum Instruction {
    Noop,
    AddX(i32)
}

impl Instruction {
    pub fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ts = s.trim().to_lowercase();
        if ts == "noop" {
            return Ok(Instruction::Noop)
        }
        if ts.starts_with("addx ") {
            match ts.trim_start_matches("addx ").parse() {
                Ok(v) => return Ok(Instruction::AddX(v)),
                Err(e) => return Err(ParseError{msg: format!("Invalid value: {}. {}", s, e)})
            };
        }

        return Err(ParseError{msg: format!("UNknown instruction: {}", s)})
    }
}