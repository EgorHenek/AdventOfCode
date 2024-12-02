mod day1;
mod day1_gold;
mod day2;
mod day2_gold;

fn main() {
    println!("Day1: {}", day1::result());
    println!("Day1 (🥇): {}", day1_gold::result());
    println!("Day2: {}", day2::result());
    println!("Day2 (🥇): {}", day2_gold::result())
}
