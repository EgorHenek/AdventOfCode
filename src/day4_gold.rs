use std::fs::File;

use crate::day4::parse_file;

fn count_xmas(letters: Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = letters.iter().map(|s| s.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    (1..height - 1)
        .flat_map(|i| (1..width - 1).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == 'A')
        .filter(|&(i, j)| {
            let patterns = [
                [grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]],
                [grid[i - 1][j + 1], grid[i][j], grid[i + 1][j - 1]],
            ];

            patterns.iter().filter(|&pattern| is_mas(*pattern)).count() == 2
        })
        .count()
}

fn is_mas(word: [char; 3]) -> bool {
    matches!(word, ['M', 'A', 'S'] | ['S', 'A', 'M'])
}

pub fn result(path: &str) -> usize {
    let file = File::open(path).unwrap();

    let letters = parse_file(file);

    count_xmas(letters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_xmas() {
        assert!(is_mas(['M', 'A', 'S']));
        assert!(is_mas(['S', 'A', 'M']));
        assert!(!is_mas(['A', 'M', 'S']));
    }

    #[test]
    fn test_example_input() {
        let result = result("data/day4_example.txt");

        assert_eq!(result, 9)
    }

    #[test]
    fn test_day4_gold() {
        let result = result("data/day4_input.txt");

        assert_eq!(result, 1998)
    }
}
