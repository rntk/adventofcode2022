use std::{fs, fmt};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors
}

fn choice_points(ch: &Choice) -> i64 {
    match ch {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3
    }
}

pub struct ChoiceParseError {
    msg: String
}

impl fmt::Display for ChoiceParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromStr for Choice {
    type Err = ChoiceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls = s.trim().to_uppercase().to_string();
        match ls.as_str() {
            "A" => Ok(Choice::Rock),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "Z" => Ok(Choice::Scissors),
            _ => Err(ChoiceParseError{msg: format!("unsupported value: {}", s)})
        }
    }
}

pub enum GameResult {
    Lost,
    Draw,
    Win
}

fn result_points(r: &GameResult) -> i64 {
    match r {
        GameResult::Lost => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6
    }
}

fn result(left: &Choice, right: &Choice) -> GameResult {
    if left == right {
        return GameResult::Draw;
    }
    match left {
        Choice::Rock => if *right == Choice::Paper { GameResult::Win} else {GameResult::Lost},
        Choice::Paper => if *right == Choice::Scissors { GameResult::Win} else {GameResult::Lost},
        Choice::Scissors => if *right == Choice::Rock { GameResult::Win} else {GameResult::Lost},
    }
}

pub fn score(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut sum: i64 = 0;
    for (i, s) in strings.enumerate() {
        if s.trim() == "" {
            continue
        }
        let chs: Vec<&str> = s.split(" ").collect();
        if chs.len() != 2 {
            return format!("Parse failed. Line: {}. Value: {}", i, s)
        }
        let left: Choice = match chs[0].parse() {
            Ok(ch) => ch,
            Err(e) => return format!("Parse failed. Line: {}. Value: {}. {}", i, s, e)
        };
        let right: Choice = match chs[1].parse() {
            Ok(ch) => ch,
            Err(e) => return format!("Parse failed. Line: {}. Value: {}. {}", i, s, e)
        };

        sum += choice_points(&right) + result_points(&result(&left, &right))
    }

    return sum.to_string();
}