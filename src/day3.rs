use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub trait Instruction {
    fn calculate(&self) -> usize;
}

pub struct MulInstruction {
    x: usize,
    y: usize,
}

impl Instruction for MulInstruction {
    fn calculate(&self) -> usize {
        self.x * self.y
    }
}

impl MulInstruction {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub fn parse_file(file: File) -> String {
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>()
        .join("")
}

fn parse_memory(memory: String) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();

    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    for (_, [digits]) in re.captures_iter(&memory).map(|c| c.extract()) {
        let digits: Vec<_> = digits
            .split(",")
            .map(|digit| digit.parse().unwrap())
            .collect();
        instructions.push(Box::new(MulInstruction::new(digits[0], digits[1])))
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
    fn test_parse_file() {
        let file = File::open("data/day3_example.txt").unwrap();

        let memory = parse_file(file);

        assert_eq!(
            memory,
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        )
    }

    #[test]
    fn test_parse_memory() {
        let file = File::open("data/day3_example.txt").unwrap();
        let memory = parse_file(file);

        let commands = parse_memory(memory);

        assert_eq!(commands.len(), 4);
        assert_eq!(commands.first().unwrap().calculate(), 8)
    }

    #[test]
    fn test_day3_example() {
        let result = result("data/day3_example.txt");

        assert_eq!(result, 161)
    }

    #[test]
    fn test_day3() {
        let result = result("data/day3_input.txt");

        assert_eq!(result, 173419328)
    }
}
