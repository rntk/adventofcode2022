mod days;

fn main() {
    println!("Day 1: Max calories - {}", days::day1::max_calories("./inputs/day1.txt"));
    println!("Day 1: Top 3 calories - {}", days::day1::top_3("./inputs/day1.txt"));
    println!("Day 2: Score - {}", days::day2::score("./inputs/day2.txt"));
    println!("Day 2: Strategy score - {}", days::day2::score_strategy("./inputs/day2.txt"));
}
