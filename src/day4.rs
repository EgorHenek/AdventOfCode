use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_file(file: File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>()
}

fn count_xmas(letters: Vec<String>) -> usize {
    let mut result = 0;

    for i in 0..letters.len() {
        for j in 0..letters.len() {
            // horizontal
            if j <= letters.len() - 4 {
                let word = letters[i].chars().skip(j).take(4).collect();

                if is_xmas(word) {
                    result += 1;
                }
            }

            // vertical
            if i <= letters.len() - 4 {
                let word: String = letters
                    .iter()
                    .skip(i)
                    .take(4)
                    .map(|row| row.chars().nth(j).unwrap())
                    .collect();

                if is_xmas(word) {
                    result += 1;
                }
            }

            // left-down diagonal
            if i <= letters.len() - 4 && j >= 3 {
                let mut word: Vec<char> = vec![];
                for skip in 0..=3 {
                    word.push(letters[i + skip].chars().nth(j - skip).unwrap());
                }

                if is_xmas(word.iter().collect()) {
                    result += 1;
                }
            }

            // right-down diagonal
            if i <= letters.len() - 4 && j <= letters.len() - 4 {
                let mut word: Vec<char> = vec![];
                for skip in 0..=3 {
                    word.push(letters[i + skip].chars().nth(j + skip).unwrap());
                }

                if is_xmas(word.iter().collect()) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn is_xmas(letters: String) -> bool {
    letters == "XMAS" || letters == "SAMX"
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
    fn test_parse_file() {
        let file = File::open("data/day4_example.txt").unwrap();

        let letters = parse_file(file);

        assert_eq!(
            letters,
            vec![
                "MMMSXXMASM",
                "MSAMXMSMSA",
                "AMXSXMAAMM",
                "MSAMASMSMX",
                "XMASAMXAMM",
                "XXAMMXXAMA",
                "SMSMSASXSS",
                "SAXAMASAAA",
                "MAMMMXMMMM",
                "MXMXAXMASX"
            ]
        )
    }

    #[test]
    fn test_is_xmas() {
        assert!(is_xmas("XMAS".to_string()));
        assert!(is_xmas("SAMX".to_string()));
        assert!(!is_xmas("MAXS".to_string()));
    }

    #[test]
    fn test_example_input() {
        let result = result("data/day4_example.txt");

        assert_eq!(result, 18)
    }

    #[test]
    fn test_day4() {
        let result = result("data/day4_input.txt");

        assert_eq!(result, 2569)
    }
}
