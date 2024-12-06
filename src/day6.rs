use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq)]
struct GuardPosition(i32, i32);

#[derive(Debug, PartialEq, Eq)]
enum GuardDirection {
    Down,
    Left,
    Right,
    Up,
}

impl TryFrom<char> for GuardDirection {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _ => Err("A symbol is not a direction"),
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: GuardPosition,
    direction: GuardDirection,
}

enum CanMoveError {
    Obstruction,
    OutOfArea,
}

impl Guard {
    fn turnaround(&mut self) {
        match self.direction {
            GuardDirection::Down => self.direction = GuardDirection::Left,
            GuardDirection::Left => self.direction = GuardDirection::Up,
            GuardDirection::Right => self.direction = GuardDirection::Down,
            GuardDirection::Up => self.direction = GuardDirection::Right,
        }
    }

    fn make_move(&mut self, area: &Area) -> Result<(), &'static str> {
        loop {
            match self.can_move(area) {
                Ok(()) => break,
                Err(CanMoveError::Obstruction) => self.turnaround(),
                Err(CanMoveError::OutOfArea) => return Err("Finish!"),
            }
        }

        match self.direction {
            GuardDirection::Down => self.position.1 += 1,
            GuardDirection::Left => self.position.0 -= 1,
            GuardDirection::Right => self.position.0 += 1,
            GuardDirection::Up => self.position.1 -= 1,
        }

        Ok(())
    }

    fn can_move(&mut self, area: &Area) -> Result<(), CanMoveError> {
        let (next_x, next_y) = self.next_position();

        self.check_area_bounds(area, next_x, next_y)?;
        self.check_obstructions(area, next_x, next_y)?;

        Ok(())
    }

    fn next_position(&self) -> (i32, i32) {
        match self.direction {
            GuardDirection::Down => (self.position.0, self.position.1 + 1),
            GuardDirection::Up => (self.position.0, self.position.1 - 1),
            GuardDirection::Left => (self.position.0 - 1, self.position.1),
            GuardDirection::Right => (self.position.0 + 1, self.position.1),
        }
    }

    fn check_area_bounds(&self, area: &Area, x: i32, y: i32) -> Result<(), CanMoveError> {
        if x < 0 || x >= area.width || y < 0 || y >= area.height {
            Err(CanMoveError::OutOfArea)
        } else {
            Ok(())
        }
    }

    fn check_obstructions(&self, area: &Area, x: i32, y: i32) -> Result<(), CanMoveError> {
        if area.obstructions[y as usize].contains(&x) {
            Err(CanMoveError::Obstruction)
        } else {
            Ok(())
        }
    }
}

pub struct Area {
    pub width: i32,
    pub height: i32,
    pub obstructions: Vec<HashSet<i32>>,
}

impl Area {
    pub fn new(width: i32, height: i32, obstructions: Vec<HashSet<i32>>) -> Self {
        Self {
            width,
            height,
            obstructions,
        }
    }
}

pub struct Input {
    pub area: Area,
    pub guard: Guard,
}

impl Input {
    fn new(area: Area, guard: Guard) -> Self {
        Self { area, guard }
    }
}

pub fn parse_file(file: File) -> Input {
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());

    let guard = lines
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| {
                GuardDirection::try_from(c).ok().map(|direction| Guard {
                    position: GuardPosition(x as i32, y as i32),
                    direction,
                })
            })
        })
        .expect("Guard not found in input");

    let obstructions = lines
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i as i32)
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    let area = Area::new(width as i32, height as i32, obstructions);
    Input::new(area, guard)
}

pub fn result(path: &str) -> usize {
    let file = File::open(path).unwrap();

    let mut input = parse_file(file);

    let mut visited: HashMap<i32, HashSet<i32>> = HashMap::new();
    visited
        .entry(input.guard.position.1)
        .and_modify(|e| {
            e.insert(input.guard.position.0);
        })
        .or_insert_with(|| {
            let mut set = HashSet::new();
            set.insert(input.guard.position.0);
            set
        });

    while input.guard.make_move(&input.area).is_ok() {
        visited
            .entry(input.guard.position.1)
            .and_modify(|e| {
                e.insert(input.guard.position.0);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(input.guard.position.0);
                set
            });
    }

    visited.iter().fold(0, |acc, (_, v)| acc + v.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let file = File::open("data/day6_example.txt").unwrap();

        let input = parse_file(file);

        assert_eq!(input.area.width, 10);
        assert_eq!(input.area.height, 10);
        assert!(input.area.obstructions[0].contains(&4));
        assert_eq!(input.guard.position, GuardPosition(4, 6));
        assert_eq!(input.guard.direction, GuardDirection::Up)
    }

    #[test]
    fn test_move() {
        let mut guard = Guard {
            position: GuardPosition(0, 0),
            direction: GuardDirection::Right,
        };
        let mut area = Area::new(3, 2, vec![HashSet::new(), HashSet::new()]);

        let _ = guard.make_move(&area);

        assert_eq!(guard.position, GuardPosition(1, 0));

        area.obstructions[0].insert(2);

        let _ = guard.make_move(&area);

        assert_eq!(guard.position, GuardPosition(1, 1));

        assert_eq!(guard.make_move(&area), Err("Finish!"))
    }

    #[test]
    fn test_day6_example() {
        let result = result("data/day6_example.txt");

        assert_eq!(result, 41)
    }

    #[test]
    fn test_day6() {
        let result = result("data/day6_input.txt");

        assert_eq!(result, 5145)
    }
}
