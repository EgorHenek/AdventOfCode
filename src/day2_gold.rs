use std::fs::File;

use crate::day2::parse_file;

const MAX_DIFF: u32 = 3;
const MIN_DIFF: u32 = 0;

fn is_valid_difference(a: u32, b: u32) -> bool {
    let diff = a.abs_diff(b);
    diff > MIN_DIFF && diff <= MAX_DIFF
}

fn is_strictly_monotonic(seq: &[u32]) -> bool {
    let ascending = seq.windows(2).all(|w| w[0] < w[1]);
    let descending = seq.windows(2).all(|w| w[0] > w[1]);
    ascending || descending
}

fn can_be_safe_after_removal(seq: &[u32]) -> bool {
    if is_strictly_monotonic(seq) && seq.windows(2).all(|w| is_valid_difference(w[0], w[1])) {
        return true;
    }

    for i in 0..seq.len() {
        let mut modified = seq.to_vec();
        modified.remove(i);

        if is_strictly_monotonic(&modified)
            && modified.windows(2).all(|w| is_valid_difference(w[0], w[1]))
        {
            return true;
        }
    }

    false
}

fn calculate_safe_reports(reports: Vec<Vec<u32>>) -> u32 {
    reports
        .iter()
        .filter(|report| can_be_safe_after_removal(report))
        .count() as u32
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
    fn test_calculate_safe_reports() {
        let file = File::open("data/day2_example.txt").unwrap();
        let nums = parse_file(file);

        let safe_reports_count = calculate_safe_reports(nums);

        assert_eq!(safe_reports_count, 4)
    }

    #[test]
    fn test_day2_gold() {
        let safe_reports_count = result();

        assert_eq!(safe_reports_count, 418)
    }
}
