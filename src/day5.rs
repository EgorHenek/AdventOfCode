use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default)]
pub struct Input {
    pub ordering_rules: Vec<(u32, u32)>,
    pub pages_to_produce: Vec<Vec<u32>>,
}

pub fn parse_file(file: File) -> Input {
    let mut input = Input::default();

    let lines = BufReader::new(file).lines();

    for line in lines {
        let line = line.unwrap();

        if line.contains("|") {
            let mut pair = line.split("|").map(|num| num.parse().unwrap());
            input
                .ordering_rules
                .push((pair.next().unwrap(), pair.next().unwrap()));
        }

        if line.contains(",") {
            let page: Vec<u32> = line.split(",").map(|num| num.parse().unwrap()).collect();
            input.pages_to_produce.push(page);
        }
    }

    input
}

pub fn calculate_ordering(rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    rules
        .iter()
        .fold(HashMap::new(), |mut counter, &(from, to)| {
            counter.entry(from).or_default().insert(to);
            counter
        })
}

fn pages_filter(page: &[u32], ordering: &HashMap<u32, HashSet<u32>>) -> bool {
    page.windows(2).all(|window| {
        ordering
            .get(&window[0])
            .is_some_and(|from| from.contains(&window[1]))
    })
}

pub fn result(path: &str) -> u32 {
    let file = File::open(path).unwrap();
    let input = parse_file(file);

    let ordering = calculate_ordering(&input.ordering_rules);

    input
        .pages_to_produce
        .iter()
        .filter(|page| pages_filter(page, &ordering))
        .map(|rule| rule[rule.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let file = File::open("data/day5_example.txt").unwrap();

        let input = parse_file(file);

        assert_eq!(
            input.ordering_rules,
            vec![
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ]
        );
        assert_eq!(
            input.pages_to_produce,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        )
    }

    #[test]
    fn test_calculate_ordering() {
        let ordering = calculate_ordering(&[(10, 13), (13, 5), (2, 13)]);

        assert!(ordering.get(&13).unwrap().contains(&5))
    }

    #[test]
    fn test_pages_filter() {
        let ordering = calculate_ordering(&[(1, 2), (2, 3), (1, 3)]);

        assert!(pages_filter(&[1, 2], &ordering));
        assert!(pages_filter(&[1, 2, 3], &ordering));
        assert!(!pages_filter(&[2, 1], &ordering));
    }

    #[test]
    fn test_day5_example() {
        let result = result("data/day5_example.txt");

        assert_eq!(result, 143)
    }

    #[test]
    fn test_day5() {
        let result = result("data/day5_input.txt");

        assert_eq!(result, 6242)
    }
}
