use std::fs::File;

use regex::Regex;

use crate::day3::{parse_file, Instruction, MulInstruction};

fn parse_memory(memory: String) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();

    let mut mul_enabled = true;

    let re = Regex::new(r"(mul)\((\d+,\d+)\)|(do)\(\)|(don't)\(\)").unwrap();

    for cap in re.captures_iter(&memory) {
        if cap.get(1).is_some() {
            if mul_enabled {
                if let Some(digits) = cap.get(2) {
                    let digits: Vec<_> = digits
                        .as_str()
                        .split(",")
                        .map(|digit| digit.parse().unwrap())
                        .collect();
                    instructions.push(Box::new(MulInstruction::new(digits[0], digits[1])));
                }
            }
        } else if cap.get(3).is_some() {
            mul_enabled = true;
        } else if cap.get(4).is_some() {
            mul_enabled = false;
        }
    }

    instructions
}

pub fn result(path: &str) -> usize {
    let file = File::open(path).unwrap();

    let memory = parse_file(file);

    let instructions = parse_memory(memory);

    instructions
        .iter()
        .map(|instruction| instruction.calculate())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_memory() {
        let file = File::open("data/day3_golden_example.txt").unwrap();
        let memory = parse_file(file);

        let commands = parse_memory(memory);

        assert_eq!(commands.len(), 2);
        assert_eq!(commands.first().unwrap().calculate(), 8)
    }

    #[test]
    fn test_day3_example() {
        let result = result("data/day3_golden_example.txt");

        assert_eq!(result, 48)
    }

    #[test]
    fn test_day3_gold() {
        let result = result("data/day3_input.txt");

        assert_eq!(result, 90669332)
    }
}
