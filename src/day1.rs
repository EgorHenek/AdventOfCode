use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_diff(nums: Vec<i32>) -> u32 {
    let (mut column1, mut column2): (Vec<i32>, Vec<i32>) =
        nums.chunks(2).map(|chunk| (chunk[0], chunk[1])).unzip();

    column1.sort_unstable();
    column2.sort_unstable();

    column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn parse_file(file: File) -> Vec<i32> {
    BufReader::new(file)
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn result() -> u32 {
    let file = File::open("data/day1_input.txt").unwrap();

    let nums = parse_file(file);

    calculate_diff(nums)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diff() {
        let nums = vec![5, 10, 15, 3];

        let result = calculate_diff(nums);

        assert_eq!(result, 7)
    }

    #[test]
    fn test_parse_file() {
        let file = File::open("data/day1_example.txt").unwrap();

        let result = parse_file(file);

        assert_eq!(*result.first().unwrap(), 3);
        assert_eq!(*result.get(1).unwrap(), 4)
    }

    #[test]
    fn test_day1_example() {
        let file = File::open("data/day1_example.txt").unwrap();

        let nums = parse_file(file);
        let diff = calculate_diff(nums);

        assert_eq!(diff, 11)
    }

    #[test]
    fn test_day1() {
        let file = File::open("data/day1_input.txt").unwrap();

        let nums = parse_file(file);
        let diff = calculate_diff(nums);

        assert_eq!(diff, 2756096)
    }
}
