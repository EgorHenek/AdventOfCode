use std::{collections::HashMap, fs::File};

use crate::day1::parse_file;

fn calculate_similarity_score(nums: Vec<i32>) -> u32 {
    let (column1, column2): (Vec<i32>, Vec<i32>) =
        nums.chunks(2).map(|chunk| (chunk[0], chunk[1])).unzip();

    let counter = column2.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let mut result = 0;

    for num in column1 {
        let count = counter.get(&num).unwrap_or(&0);
        result += count * num;
    }
    result as u32
}

pub fn result() -> u32 {
    let file = File::open("data/day1_input.txt").unwrap();

    let nums = parse_file(file);

    calculate_similarity_score(nums)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_similarity_score() {
        let nums = vec![5, 10, 5, 5];

        let result = calculate_similarity_score(nums);

        assert_eq!(result, 10)
    }

    #[test]
    fn test_day1_example() {
        let file = File::open("data/day1_example.txt").unwrap();

        let nums = parse_file(file);
        let diff = calculate_similarity_score(nums);

        assert_eq!(diff, 31)
    }

    #[test]
    fn test_day1_gold() {
        let file = File::open("data/day1_input.txt").unwrap();

        let nums = parse_file(file);
        let diff = calculate_similarity_score(nums);

        assert_eq!(diff, 23117829)
    }
}
