use std::fs;

pub fn max_calories(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut sum: i64 = 0;
    let mut max: i64 = 0;
    for (i, s) in strings.enumerate() {
        if s.trim() == "" {
            if sum >= max {
                max = sum;
            }
            sum = 0;
            continue
        }
        let num: i64 = match s.parse() {
            Ok(n) => n,
            Err(e) => return format!("Parse failed. Line: {}. Value: {}. {}", i, s, e)
        };
        sum += num
    }
    if sum >= max {
        max = sum
    }


    return max.to_string()
}

pub fn top_3(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let strings = input.split("\n");
    let mut sum: i64 = 0;
    let mut max: [i64; 3] = [0; 3];
    for (i, s) in strings.enumerate() {
        if s.trim() == "" {
            let pos = pos_min(&max);
            if sum >= max[pos] {
                max[pos] = sum
            }
            sum = 0;
            continue
        }
        let num: i64 = match s.parse() {
            Ok(n) => n,
            Err(e) => return format!("Parse failed. Line: {}. Value: {}. {}", i, s, e)
        };
        sum += num
    }
    let pos = pos_min(&max);
    if sum >= max[pos] {
        max[pos] = sum
    }

    return max.iter().sum::<i64>().to_string();
}

fn pos_min(numbers: &[i64; 3]) -> usize {
    let mut min_pos: usize = 0;
    for (pos, v) in numbers.iter().enumerate() {
        if numbers[min_pos] >= *v  {
            min_pos = pos
        }
    }

    return min_pos
}