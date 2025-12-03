use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day03.txt").expect("Failed to read input file");
    let sol1 = part1(&input);
    let sol2 = part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let batteries = parse_line_into_batteries(line);
            find_biggest_joltage(&batteries, 2)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let batteries = parse_line_into_batteries(line);
            find_biggest_joltage(&batteries, 12)
        })
        .sum()
}

fn find_biggest_joltage(batteries: &Vec<u64>, num_digits_to_combine: usize) -> u64 {
    let mut left_to_remove = batteries.len() - num_digits_to_combine;
    let mut stack_of_digits = Vec::new();

    for &digit in batteries {
        while left_to_remove > 0
            && !stack_of_digits.is_empty()
            && *stack_of_digits.last().unwrap() < digit
        {
            stack_of_digits.pop();
            left_to_remove -= 1;
        }
        stack_of_digits.push(digit);
    }
    let max_joltage = stack_of_digits
        .iter()
        .take(num_digits_to_combine)
        .fold(0, |acc, &d| acc * 10 + d);
    max_joltage
}

fn parse_line_into_batteries(line: &str) -> Vec<u64> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

//test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_biggest_joltage() {
        let test_string = "987654321111111";
        let batteries = parse_line_into_batteries(test_string);
        let result = find_biggest_joltage(&batteries, 2);
        assert_eq!(result, 98);
        let result = find_biggest_joltage(&batteries, 12);
        assert_eq!(result, 987654321111);

        let test_string = "811111111111119";
        let batteries = parse_line_into_batteries(test_string);
        let result = find_biggest_joltage(&batteries, 2);
        assert_eq!(result, 89);
        let result = find_biggest_joltage(&batteries, 12);
        assert_eq!(result, 811111111119);

        let test_string = "234234234234278";
        let batteries = parse_line_into_batteries(test_string);
        let result = find_biggest_joltage(&batteries, 2);
        assert_eq!(result, 78);
        let result = find_biggest_joltage(&batteries, 12);
        assert_eq!(result, 434234234278);

        let test_string = "818181911112111";
        let batteries = parse_line_into_batteries(test_string);
        let result = find_biggest_joltage(&batteries, 2);
        assert_eq!(result, 92);
        let result = find_biggest_joltage(&batteries, 12);
        assert_eq!(result, 888911112111);
    }
}
