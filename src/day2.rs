use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_DIFF: u32 = 3;
const MIN_DIFF: u32 = 0;

fn is_valid_difference(a: u32, b: u32) -> bool {
    let diff = a.abs_diff(b);
    diff > MIN_DIFF && diff <= MAX_DIFF
}

fn calculate_safe_reports(reports: Vec<Vec<u32>>) -> u32 {
    reports
        .iter()
        .filter(|report| {
            let is_monotonic = report.windows(2).all(|window| window[0] > window[1])
                || report.windows(2).all(|window| window[0] < window[1]);

            is_monotonic
                && report
                    .windows(2)
                    .all(|window| is_valid_difference(window[0], window[1]))
        })
        .count() as u32
}

pub fn parse_file(file: File) -> Vec<Vec<u32>> {
    BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn result() -> u32 {
    let file = File::open("data/day2_input.txt").unwrap();

    let nums = parse_file(file);

    calculate_safe_reports(nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let file = File::open("data/day2_example.txt").unwrap();

        let nums = parse_file(file);

        assert_eq!(nums.len(), 6);
        assert_eq!(nums[0].len(), 5);
        assert_eq!(nums[0][0], 7);
        assert_eq!(nums[0][4], 1);
    }

    #[test]
    fn test_calculate_safe_reports() {
        let file = File::open("data/day2_example.txt").unwrap();
        let nums = parse_file(file);

        let safe_reports_count = calculate_safe_reports(nums);

        assert_eq!(safe_reports_count, 2)
    }
    #[test]
    fn test_day2() {
        let safe_reports_count = result();

        assert_eq!(safe_reports_count, 359)
    }
}
