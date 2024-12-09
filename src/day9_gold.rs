use std::fs::File;

use crate::day9::parse_file;

fn calculate_fragmentation(memory: &mut [isize]) -> usize {
    let mut right = memory.len() - 1;

    while right > 0 {
        while memory[right] == -1 {
            right -= 1;
        }

        let mut current_block_size = 0;
        let current_id = memory[right];

        while right > 0 && memory[right] == current_id {
            current_block_size += 1;
            right -= 1;
        }

        let mut left = 0;
        let mut current_free_block_size = 0;
        while left < right {
            if memory[left] == -1 {
                current_free_block_size += 1;
            } else {
                current_free_block_size = 0;
            }

            if current_free_block_size == current_block_size {
                (left + 1 - current_free_block_size..=left).for_each(|i| {
                    memory[i] = current_id;
                });

                (right + 1..=right + current_block_size).for_each(|i| {
                    memory[i] = -1;
                });
                break;
            }

            left += 1;
        }
    }

    memory
        .iter()
        .enumerate()
        .filter(|(_, id)| **id != -1)
        .map(|(index, id)| index * *id as usize)
        .sum()
}

pub fn result(path: &str) -> usize {
    let file = File::open(path).unwrap();

    let mut memory = parse_file(file);

    calculate_fragmentation(&mut memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut memory = vec![
            0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5,
            5, -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
        ];

        let result = calculate_fragmentation(&mut memory);

        assert_eq!(result, 2858)
    }
}
