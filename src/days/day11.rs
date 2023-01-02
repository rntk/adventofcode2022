use std::fs;
use std::str::FromStr;

use crate::days::day2::ParseError;

pub fn level(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey= Monkey::new();
    for (line, s) in input.split("\n").enumerate() {
        let st = s.trim().to_lowercase();
        if st == "" {
            continue
        }
        if st.starts_with("monkey") {
            if line > 0 {
                monkeys.push(monkey);
                monkey = Monkey::new();
            }
            continue
        }
        if st.starts_with("starting") {
            if let Err(e) = monkey.parse_items(&st) {
                return format!("Failed to parse items: {} {}. {}", line, s, e)
            }
        }
        if st.starts_with("operation") {
            if let Err(e) = monkey.parse_operation(&st) {
                return format!("Failed to parse operation: {} {}. {}", line, s, e)
            }
        }
        if st.starts_with("test") {
            if let Err(e) = monkey.parse_test(&st) {
                return format!("Failed to parse test: {} {}. {}", line, s, e)
            }
        }
        if st.starts_with("if") {
            if let Err(e) = monkey.parse_conditions(&st) {
                return format!("Failed to parse conditions: {} {}. {}", line, s, e)
            }
        }
    }
    monkeys.push(monkey);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            if monkeys[i].items.len() == 0 {
                continue
            }
            for j in 0..monkeys[i].items.len() {
                let mut level = monkeys[i].items[j];
                level = match &monkeys[i].new_level {
                    NewLevel::None => return format!("Monkey new level is None: {}", i),
                    NewLevel::Mult(vr) => match vr {
                        OperationVar::Number(n) => level * n,
                        OperationVar::Old => level * level
                    },
                    NewLevel::Plus(vr) => match vr {
                        OperationVar::Number(n) => level + n,
                        OperationVar::Old => level + level
                    },
                };
                level = (level as f32 / 3 as f32).floor() as i32;
                let test = (level % monkeys[i].test) == 0;
                for y in 0..monkeys[i].monkeys.len() {
                    let (t, new_monkey) = monkeys[i].monkeys[y];
                    if t == test {
                        monkeys[new_monkey].items.push(level);
                        break
                    }
                }
            }
            monkeys[i].inspects_n += monkeys[i].items.len() as u32;
            monkeys[i].items.clear();
        }
    }
    monkeys.sort_by(|a, b| b.inspects_n.cmp(&a.inspects_n));
    let b_level = monkeys[0].inspects_n * monkeys[1].inspects_n;

    return b_level.to_string()
}

pub fn level1(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut monkeys: Vec<Monkey1> = vec![];
    let mut monkey= Monkey1::new();
    for (line, s) in input.split("\n").enumerate() {
        let st = s.trim().to_lowercase();
        if st == "" {
            continue
        }
        if st.starts_with("monkey") {
            if line > 0 {
                monkeys.push(monkey);
                monkey = Monkey1::new();
            }
            continue
        }
        if st.starts_with("starting") {
            if let Err(e) = monkey.parse_items(&st) {
                return format!("Failed to parse items: {} {}. {}", line, s, e)
            }
        }
        if st.starts_with("operation") {
            if let Err(e) = monkey.parse_operation(&st) {
                return format!("Failed to parse operation: {} {}. {}", line, s, e)
            }
        }
        if st.starts_with("test") {
            if let Err(e) = monkey.parse_test(&st) {
                return format!("Failed to parse test: {} {}. {}", line, s, e)
            }
        }
        if st.starts_with("if") {
            if let Err(e) = monkey.parse_conditions(&st) {
                return format!("Failed to parse conditions: {} {}. {}", line, s, e)
            }
        }
    }
    monkeys.push(monkey);
    let mut new_div = 1;
    // hint from other participants
    for m in &monkeys {
        new_div *= m.test;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            if monkeys[i].items.len() == 0 {
                continue
            }
            for j in 0..monkeys[i].items.len() {
                let mut level = monkeys[i].items[j];
                level = match &monkeys[i].new_level {
                    NewLevel::None => return format!("Monkey new level is None: {}", i),
                    NewLevel::Mult(vr) => match vr {
                        OperationVar::Number(n) => level * *n as i128,
                        OperationVar::Old => level * level
                    },
                    NewLevel::Plus(vr) => match vr {
                        OperationVar::Number(n) => level + *n as i128,
                        OperationVar::Old => level + level
                    },
                };
                level %= new_div;
                let test = (level % monkeys[i].test) == 0;
                for y in 0..monkeys[i].monkeys.len() {
                    let (t, new_monkey) = monkeys[i].monkeys[y];
                    if t == test {
                        monkeys[new_monkey].items.push(level);
                        break
                    }
                }
            }
            monkeys[i].inspects_n += monkeys[i].items.len() as u64;
            monkeys[i].items.clear();
        }
    }
    monkeys.sort_by(|a, b| b.inspects_n.cmp(&a.inspects_n));
    let b_level = monkeys[0].inspects_n * monkeys[1].inspects_n;

    return b_level.to_string()
}

#[derive(Debug)]
enum OperationVar {
    Old,
    Number(i32)
}

#[derive(Debug)]
enum NewLevel {
    Plus(OperationVar),
    Mult(OperationVar),
    None
}

impl FromStr for NewLevel {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<&str> = s.trim().trim_start_matches("operation: new = ").split(" ").collect();
        if vals.len() != 3 {
            return Err(ParseError{msg: "Must be 3 elements".to_string()})
        }
        let op_var = match vals[2] {
            "old" => OperationVar::Old,
            _ => match vals[2].parse() {
                Ok(v) => OperationVar::Number(v),
                Err(e) => return Err(ParseError{msg: format!("Invalid operation var: {}. {}", vals[2], e)})
            }
        };
        let new_l =  match vals[1] {
            "+" => NewLevel::Plus(op_var),
            "*" => NewLevel::Mult(op_var),
            _ => return Err(ParseError{msg: format!("Invalid operation: {}", vals[1])})
        };

        return Ok(new_l)
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    test: i32,
    monkeys: Vec<(bool, usize)>,
    new_level: NewLevel,
    inspects_n: u32
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey{
            items: vec![],
            test: 0,
            monkeys: vec![],
            new_level: NewLevel::None,
            inspects_n: 0
        }
    }

    pub fn parse_items(&mut self, s: &str) -> Result<(), ParseError> {
        let items_s = s.trim_start_matches("starting items:").trim().split(",");
        for si in items_s.into_iter() {
            let lv: i32 = match si.trim().parse() {
                Ok(l) => l,
                Err(e) => return Err(ParseError{msg: format!("Failed to parse item: {}. {}", si, e)})
            };
            self.items.push(lv);
        }

        return Ok(())
    }

    pub fn parse_operation(&mut self, s: &str) -> Result<(), ParseError> {
        self.new_level = match s.parse() {
            Ok(nl) => nl,
            Err(e) => return Err(ParseError{msg: format!("Failed to parse operation: {}. {}", s, e)})
        };

        return Ok(())
    }

    pub fn parse_test(&mut self, s: &str) -> Result<(), ParseError> {
        self.test = match s.trim_start_matches("test: divisible by").trim().parse() {
            Ok(d) => d,
            Err(e) => return Err(ParseError{msg: format!("Failed to parse test: {}. {}", s, e)})
        };

        return Ok(())
    }


    pub fn parse_conditions(&mut self, s: &str) -> Result<(), ParseError> {
        let cond = s.trim_start_matches("if").trim();
        let test_res: bool;
        if cond.starts_with("true") || cond.starts_with("false") {
            test_res = cond.starts_with("true")
        } else {
            return Err(ParseError{msg: format!("Invalid test: {}", s)})
        }
        match cond.rfind(" ") {
            Some(pos) => match cond[pos..].trim().parse() {
                Ok(v) => self.monkeys.push((test_res, v)),
                Err(e) => return Err(ParseError{msg: format!("Invalid test value: {}. {}", s, e)})
            },
            None => return Err(ParseError{msg: format!("Invalid test monkey: {}", s)})
        }

        return Ok(())
    }
}

#[derive(Debug)]
struct Monkey1 {
    items: Vec<i128>,
    test: i128,
    monkeys: Vec<(bool, usize)>,
    new_level: NewLevel,
    inspects_n: u64
}

impl Monkey1 {
    pub fn new() -> Monkey1 {
        Monkey1{
            items: vec![],
            test: 0,
            monkeys: vec![],
            new_level: NewLevel::None,
            inspects_n: 0
        }
    }

    pub fn parse_items(&mut self, s: &str) -> Result<(), ParseError> {
        let items_s = s.trim_start_matches("starting items:").trim().split(",");
        for si in items_s.into_iter() {
            let lv: i128 = match si.trim().parse() {
                Ok(l) => l,
                Err(e) => return Err(ParseError{msg: format!("Failed to parse item: {}. {}", si, e)})
            };
            self.items.push(lv);
        }

        return Ok(())
    }

    pub fn parse_operation(&mut self, s: &str) -> Result<(), ParseError> {
        self.new_level = match s.parse() {
            Ok(nl) => nl,
            Err(e) => return Err(ParseError{msg: format!("Failed to parse operation: {}. {}", s, e)})
        };

        return Ok(())
    }

    pub fn parse_test(&mut self, s: &str) -> Result<(), ParseError> {
        self.test = match s.trim_start_matches("test: divisible by").trim().parse() {
            Ok(d) => d,
            Err(e) => return Err(ParseError{msg: format!("Failed to parse test: {}. {}", s, e)})
        };

        return Ok(())
    }


    pub fn parse_conditions(&mut self, s: &str) -> Result<(), ParseError> {
        let cond = s.trim_start_matches("if").trim();
        let test_res: bool;
        if cond.starts_with("true") || cond.starts_with("false") {
            test_res = cond.starts_with("true")
        } else {
            return Err(ParseError{msg: format!("Invalid test: {}", s)})
        }
        match cond.rfind(" ") {
            Some(pos) => match cond[pos..].trim().parse() {
                Ok(v) => self.monkeys.push((test_res, v)),
                Err(e) => return Err(ParseError{msg: format!("Invalid test value: {}. {}", s, e)})
            },
            None => return Err(ParseError{msg: format!("Invalid test monkey: {}", s)})
        }

        return Ok(())
    }
}