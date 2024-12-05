use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
};

use crate::day5::{calculate_ordering, parse_file};

fn pages_filter(page: &[u32], ordering: &HashMap<u32, HashSet<u32>>) -> bool {
    !page.windows(2).all(|window| {
        ordering
            .get(&window[0])
            .is_some_and(|from| from.contains(&window[1]))
    })
}

fn sort_page<'a>(
    page: &'a mut Vec<u32>,
    ordering: &HashMap<u32, HashSet<u32>>,
) -> &'a mut Vec<u32> {
    page.sort_by(|from, to| {
        if ordering.get(from).map_or(false, |set| set.contains(to)) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    page
}

pub fn result(path: &str) -> u32 {
    let file = File::open(path).unwrap();
    let mut input = parse_file(file);

    let ordering = calculate_ordering(&input.ordering_rules);

    input
        .pages_to_produce
        .iter_mut()
        .filter(|page| pages_filter(page, &ordering))
        .map(|page| sort_page(page, &ordering))
        .map(|page| page[page.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pages_filter() {
        let ordering = calculate_ordering(&[(1, 2), (2, 3), (1, 3)]);

        assert!(!pages_filter(&[1, 2], &ordering));
        assert!(!pages_filter(&[1, 2, 3], &ordering));
        assert!(pages_filter(&[2, 1], &ordering));
    }

    #[test]
    fn test_day5_example() {
        let result = result("data/day5_example.txt");

        assert_eq!(result, 123)
    }

    #[test]
    fn test_day5_gold() {
        let result = result("data/day5_input.txt");

        assert_eq!(result, 5169)
    }
}
