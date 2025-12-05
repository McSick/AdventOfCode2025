use std::fs::read_to_string;
use crate::{Solution, SolutionPair, etc::DOUBLE_NEWLINE};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...

    let input_string = read_to_string("input/day05.txt").expect("Failed to read input");
    let (mut database, queries) = parse_input(&input_string);
    database.merge_fresh_ranges();
    let sol1 = how_many_ingredients_are_fresh(&database, &queries);
    let sol2 = number_of_ids_in_ranges(&database);

    (Solution::from(sol1), Solution::from(sol2))
}
#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}
struct IngredientDatabase {
    fresh_ingredients: Vec<Range>,
}
impl IngredientDatabase {
    fn new(ranges: Vec<Range>) -> Self {
        Self {
            fresh_ingredients: ranges,
        }
    }
    fn is_fresh(&self, ingredient: u64) -> bool {
        for range in &self.fresh_ingredients {
            if ingredient >= range.start && ingredient <= range.end {
                return true;
            }
        }
        false
    }
    fn merge_fresh_ranges(&mut self) {
        self.fresh_ingredients.sort_by_key(|r| r.start);
        let mut merged: Vec<Range> = Vec::new();
        for range in &self.fresh_ingredients {
            if let Some(last) = merged.last_mut() {
                if range.start <= last.end + 1 {
                    last.end = last.end.max(range.end);
                } else {
                    merged.push(Range {
                        start: range.start,
                        end: range.end,
                    });
                }
            } else {
                merged.push(Range {
                    start: range.start,
                    end: range.end,
                });
            }
        }
        self.fresh_ingredients = merged;
    }
}
fn number_of_ids_in_ranges(database: &IngredientDatabase) -> u64 {
    database
        .fresh_ingredients
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum()
}
fn how_many_ingredients_are_fresh(database: &IngredientDatabase, queries: &Vec<u64>) -> u64 {
    queries.iter().filter(|&&q| database.is_fresh(q)).count() as u64
}
fn parse_input(input: &str) -> (IngredientDatabase, Vec<u64>) {
    let mut parts = input.split(DOUBLE_NEWLINE);
    let database = parts.next().unwrap();
    let queries = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let ranges = database
        .lines()
        .map(|line| {
            let mut range_parts = line.split('-');
            let start = range_parts.next().unwrap().parse::<u64>().unwrap();
            let end = range_parts.next().unwrap().parse::<u64>().unwrap();
            Range { start, end }
        })
        .collect::<Vec<Range>>();

    (IngredientDatabase::new(ranges), queries)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    #[test]
    fn test_how_many_ingredients_are_fresh() {
        let (mut database, queries) = parse_input(EXAMPLE_INPUT);
        database.merge_fresh_ranges();
        assert_eq!(database.fresh_ingredients.len(), 2);
        let fresh_count = how_many_ingredients_are_fresh(&database, &queries);
        assert_eq!(fresh_count, 3);
        let total_ids = number_of_ids_in_ranges(&database);
        assert_eq!(total_ids, 14);
    }
}
