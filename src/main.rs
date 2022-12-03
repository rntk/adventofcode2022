mod days;

fn main() {
    println!("Day 1: Max calories - {}", days::day1::max_calories("./inputs/day1.txt"));
    println!("Day 1: Top 3 calories - {}", days::day1::top_3("./inputs/day1.txt"));
}
