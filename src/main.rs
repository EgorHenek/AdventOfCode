mod day1;
mod day1_gold;
mod day2;
mod day2_gold;
mod day3;
mod day3_gold;
mod day4;
mod day4_gold;

fn main() {
    println!("Day1: {}", day1::result());
    println!("Day1 (🥇): {}", day1_gold::result());
    println!("Day2: {}", day2::result());
    println!("Day2 (🥇): {}", day2_gold::result());
    println!("Day3: {}", day3::result("data/day3_input.txt"));
    println!("Day3 (🥇): {}", day3_gold::result("data/day3_input.txt"));
    println!("Day4: {}", day4::result("data/day4_input.txt"));
    println!("Day4 (🥇): {}", day4_gold::result("data/day4_input.txt"));
}
