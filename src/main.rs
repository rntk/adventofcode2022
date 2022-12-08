mod days;

use std::env;
use std::process::exit;

fn day1() {
    println!("Day 1: Max calories - {}", days::day1::max_calories("./inputs/day1.txt"));
    println!("Day 1: Top 3 calories - {}", days::day1::top_3("./inputs/day1.txt"));
}

fn day2() {
    println!("Day 2: Score - {}", days::day2::score("./inputs/day2.txt"));
    println!("Day 2: Strategy score - {}", days::day2::score_strategy("./inputs/day2.txt"));
}

fn day3() {
    println!("Day 3: Sum - {}", days::day3::sum_priorities("./inputs/day3.txt"));
    println!("Day 3: Sum groups- {}", days::day3::sum_groups_priorities("./inputs/day3.txt"));
}

fn day4() {
    println!("Day 4: Ranges - {}", days::day4::count_ranges("./inputs/day4.txt"));
    println!("Day 4: Overlapped ranges - {}", days::day4::count_overlapped_ranges("./inputs/day4.txt"));
}

fn day5() {
    println!("Day 4: Letters - {}", days::day5::letters("./inputs/day5.txt"));
    println!("Day 4: Letters ordered- {}", days::day5::letters_order("./inputs/day5.txt"));
}

fn day6() {
    println!("Day 6: Letters - {}", days::day6::number("./inputs/day6.txt"));
}

fn main() {
    let days_list: Vec<fn()> = vec![
        day1,
        day2,
        day3,
        day4,
        day5,
        day6
    ];
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong arguments number");
        exit(1)
    }
    let day: usize = match args[1].parse() {
        Ok(d) => d,
        Err(e) => {
            println!("Invalid day number: {}", e);
            exit(1)
        }
    };
    if (day == 0) || (day - 1 >= days_list.len()) {
        println!("No such day: {}", day);
        exit(1)
    };
    days_list[day - 1]()
}
