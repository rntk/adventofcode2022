use std::fs;

pub fn number(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut marker: Vec<char> = vec![];
    for (i, ch) in input.chars().enumerate() {
        if let Some(pos) = find(&marker, &ch) {
            for _ in 0..pos+1 {
                marker.remove(0);
            }
        }
        marker.push(ch);
        if marker.len() == 4 {
            return (i + 1).to_string();
        }
    }

    return "-1".to_string()
}

fn find(list: &Vec<char>, ch: &char) -> Option<usize> {
    for (i, chr) in list.iter().enumerate() {
        if ch == chr {
            return Some(i)
        }
    }

    return None
}