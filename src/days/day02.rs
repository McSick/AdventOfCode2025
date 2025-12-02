use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day02.txt").unwrap();
    let ranges = parse_input(&input);
    let sol1 = sum_repeated_numbers(&ranges, Some(2));
    let sol2 = sum_repeated_numbers(&ranges, None);

    (Solution::from(sol1), Solution::from(sol2))
}
struct Range {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|s| {
            let mut parts = s.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect()
}
fn sum_repeated_numbers(ranges: &Vec<Range>, exact_times: Option<u64>) -> u64 {
    let mut total: u64 = 0;
    for range in ranges {
        for n in range.start..=range.end {
            let s = n.to_string();
            if is_reapted(&s, exact_times) {
                total += n;
            }
        }
    }
    total
}

fn is_reapted(s: &str, exact_times: Option<u64>) -> bool {
    let max_str_len = s.len() / 2;
    for n in 1..=max_str_len {
        let subset = &s[..n];
        let count = s.matches(subset).count();
        if count * subset.len() == s.len() {
            if let Some(exact) = exact_times {
                if count as u64 == exact {
                    return true;
                }
            } else {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_repeated() {
        assert!(is_reapted("11", Some(2)));
        assert!(is_reapted("1188511885", Some(2)));
        assert!(is_reapted("222222", Some(2)));
        assert!(is_reapted("abcabc", Some(2)));
        assert!(!is_reapted("1188511890", Some(2)));
        assert!(is_reapted("111", None));
    }
    #[test]
    fn test_sum_repeated_numbers() {
        let ranges = parse_input(TEST_INPUT);
        let total = sum_repeated_numbers(&ranges, Some(2));
        assert!(total == 1227775554);
        let total2 = sum_repeated_numbers(&ranges, None);
        assert!(total2 == 4174379265);
    }
}
