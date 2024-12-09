use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_fragmentation(memory: Vec<isize>) -> usize {
    let mut left = 0;
    let mut right = memory.len() - 1;
    let mut result = 0;

    while left <= right {
        while memory[right] == -1 {
            right -= 1;
        }
        if memory[left] == -1 {
            result += memory[right] as usize * left;
            right -= 1;
        } else {
            result += memory[left] as usize * left;
        }
        left += 1;
    }
    result
}

pub fn parse_file(file: File) -> Vec<isize> {
    let reader = BufReader::new(file);

    let blocks = reader
        .lines()
        .flat_map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .enumerate();

    let mut memory = Vec::new();
    for (id, size) in blocks {
        if id % 2 == 0 {
            for _ in 0..size {
                memory.push((id / 2) as isize);
            }
        } else {
            for _ in 0..size {
                memory.push(-1);
            }
        }
    }

    memory
}

pub fn result(path: &str) -> usize {
    let file = File::open(path).unwrap();

    let memory = parse_file(file);

    calculate_fragmentation(memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let file = File::open("data/day9_example.txt").unwrap();

        let result = parse_file(file);

        assert_eq!(
            result,
            vec![
                0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5,
                5, 5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9
            ]
        )
    }

    #[test]
    fn test_calculate() {
        let memory = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];

        let result = calculate_fragmentation(memory);

        assert_eq!(result, 1928)
    }

    #[test]
    fn test_day9() {
        let result = result("data/day9_input.txt");

        assert_eq!(result, 6200294120911)
    }
}
