use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("input/day01.txt").expect("Failed to read input file");
    let lock: Lock = turn_the_lock(&input);
    (
        Solution::from(lock.matches_0),
        Solution::from(lock.passes_0),
    )
}
const MAX_LOCK_NUMBER: i32 = 99;
struct Lock {
    max_number: i32,
    current_position: i32,
    matches_0: i32,
    passes_0: i32,
}
impl Lock {
    fn new(max_number: i32) -> Self {
        Lock {
            max_number: max_number + 1, // Range is 0..max_number inclusive
            current_position: 50,
            matches_0: 0,
            passes_0: 0,
        }
    }
    fn fit_in_bounds(&self, number: i32) -> i32 {
        ((number % self.max_number) + self.max_number) % self.max_number
    }
    fn turn_left(&mut self, dist: i32) {
        self.passes_0 += dist / self.max_number;
        if self.current_position != 0 && dist % self.max_number >= self.current_position {
            self.passes_0 += 1;
        }

        self.current_position = self.fit_in_bounds(self.current_position - dist);

        if self.current_position == 0 {
            self.matches_0 += 1;
        }
    }
    fn turn_right(&mut self, dist: i32) {
        self.passes_0 += dist / self.max_number;
        if (self.current_position + (dist % self.max_number)) >= self.max_number {
            self.passes_0 += 1;
        }

        self.current_position = self.fit_in_bounds(self.current_position + dist);

        if self.current_position == 0 {
            self.matches_0 += 1;
        }
    }
}

fn turn_the_lock(input: &str) -> Lock {
    let mut lock = Lock::new(MAX_LOCK_NUMBER);
    input.lines().for_each(|line| {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        let (turn, dist) = line.split_at(1);
        let dist: i32 = dist.parse().unwrap();

        match turn {
            "L" => {
                lock.turn_left(dist);
            }
            "R" => {
                lock.turn_right(dist);
            }
            _ => panic!("Invalid turn direction: {}", turn),
        }
    });
    lock
}
///////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_the_lock() {
        let input: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let lock = turn_the_lock(input);
        println!("Matches 0: {}", lock.matches_0);
        println!("Passes 0: {}", lock.passes_0);
        assert!(lock.matches_0 == 3);
        assert!(lock.passes_0 == 6);
        let input2: &str = "R1000";
        let lock2 = turn_the_lock(input2);
        println!("Matches 0: {}", lock2.matches_0);
        println!("Passes 0: {}", lock2.passes_0);
        assert!(lock2.passes_0 == 10);
    }
}
