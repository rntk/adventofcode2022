use std::fs;
use std::str::FromStr;
use crate::days::day2::ParseError;

#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges: Vec<&str> = s.trim().split("-").collect();
        if edges.len() != 2 {
            return Err(ParseError{msg: format!("Invalid range - {}", s)})
        }
        let r = Range{start:edges[0].parse()?, end: edges[1].parse()?};
        if r.start > r.end {
            return Err(ParseError{msg: format!("Invalid range - {}", s)})
        }

        return Ok(r)
    }
}

impl Range {
    pub fn contains(&self, r: &Range) -> bool {
        if self.start > r.start {
            return false
        }
        if self.end < r.end {
            return false
        }

        return true
    }
}

pub fn count_ranges(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut sum: i64 = 0;
    for (line, s) in strings.enumerate() {
        if s.trim() == "" {
            continue;
        }
        let ranges: Vec<&str> = s.split(",").collect();
        if ranges.len() != 2 {
            return format!("Invalid: {} - {}", line, s)
        }
        let r1: Range = match ranges[0].parse() {
            Ok(r) => r,
            Err(e) => return format!("Left range parse error: {} - {} - {}", line, s, e)
        };
        let r2: Range = match ranges[1].parse() {
            Ok(r) => r,
            Err(e) => return format!("Right range parse error: {} - {} - {}", line, s, e)
        };
        if r1.contains(&r2) || r2.contains(&r1) {
            sum += 1;
        }
    }

    return sum.to_string()
}
