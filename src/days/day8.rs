use std::collections::HashMap;
use std::fs;

pub fn visible(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut field : Vec<Vec<i8>> = vec![];
    for (x, s) in input.split("\n").enumerate() {
        if s.trim() == "" {
            continue
        }
        if field.len() <= x {
            field.push(vec![]);
        }
        for ch in s.chars() {
            let height: i8 = match ch.to_string().parse() {
                Ok(v) => v,
                Err(e) => return format!("Height parsing failed. Line {}. {}. {}. {}", x, s, ch, e)
            };
            field[x].push(height)
        }
    }
    let mut visible = 0;
    for (x, cols) in field.iter().enumerate() {
        for (y, _) in cols.iter().enumerate() {
            if is_visible(&field, x, y) {
                visible += 1
            }
        }
    }

    return visible.to_string()
}

fn is_visible(field: &Vec<Vec<i8>>, x: usize, y : usize) -> bool {
    if x == 0 {
        return true
    }
    if y == 0 {
        return true
    }
    if x == field.len() - 1 {
        return true
    }
    if y == field[0].len() - 1 {
        return true
    }
    let height = field[x][y];
    let mut max: HashMap<bool, i8> = HashMap::new();
    for (i, cols) in field.iter().enumerate() {
        if i == x {
            continue
        }
        let part = i < x as usize;
        if cols[y] >= height {
            max.insert(part, cols[y]);
        }
    }
    if max.len() <= 1 {
        return true
    }
    max.clear();
    for (i, h) in field[x].iter().enumerate() {
        if i == y {
            continue
        }
        let part = i < y as usize;
        if *h >= height {
            max.insert(part, *h);
        }
    }
    if max.len() <= 1 {
        return true
    }

    return false
}