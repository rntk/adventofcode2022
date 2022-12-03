use std::fs;
use std::collections::HashMap;

fn char_to_priority(ch: &u8) -> Option<u8> {
    if *ch < 65 {
        return None
    }
    if *ch > 122 {
        return None
    }
    if *ch < 91 {
        return Some(ch - 38)
    }

    return Some(ch - 96)
}

pub fn sum_priorities(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut sum: i64 = 0;
    for (i, s) in strings.enumerate() {
        if s.trim() == "" {
            continue;
        }
        let middle = s.len() / 2;
        let mut chars = HashMap::new();
        for (pos, ch) in s.as_bytes().iter().enumerate() {
            if chars.contains_key(ch) {
                if pos < middle {
                    continue
                }
                let (prev_pos, _): (usize, u8) = *chars.get(ch).unwrap();
                if prev_pos >= middle {
                    continue
                }
                let prior = match char_to_priority(ch) {
                    Some(p) => p,
                    None => return format!("unupported type: Line - {}. Column: {}. Value: {}", i, pos, s)
                };
                chars.insert(ch, (pos, prior));
                continue;
            }
            chars.insert(ch, (pos, 0));
        }
        for (_, (_, prior)) in chars.iter() {
            sum += *prior as i64
        }
    }

    return sum.to_string()
}