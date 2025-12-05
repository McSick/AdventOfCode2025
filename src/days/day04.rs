use crate::{Solution, SolutionPair, etc::Grid};
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let grid_string = read_to_string("input/day04.txt").expect("Failed to read input");
    let grid = Grid::from_str(&grid_string);
    let sol1 = part1(grid.clone());
    let sol2 = part2(grid, 0);

    (Solution::from(sol1), Solution::from(sol2))
}
fn part1(grid: Grid<char>) -> u32 {
    grid.enumerate()
        .filter(|(_, v)| *v == &'@')
        .map(|(pos, _)| {
            pos.neighbors_diag()
                .iter()
                .map(|neighbor_pos| {
                    if grid.get(*neighbor_pos) == Some(&'@') {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .filter(|&count: &u32| count < 4)
        .count() as u32
}
fn part2(mut grid: Grid<char>, mut removed: u32) -> u32 {
    let to_remove: Vec<_> = grid
        .enumerate()
        .filter(|(_, v)| *v == &'@')
        .filter_map(|(pos, _)| {
            let neighbor_count: u32 = pos
                .neighbors_diag()
                .iter()
                .map(|neighbor_pos| {
                    if grid.get(*neighbor_pos) == Some(&'@') {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            if neighbor_count < 4 { Some(pos) } else { None }
        })
        .collect();
    if to_remove.is_empty() {
        return removed;
    }
    for pos in to_remove {
        grid.set(pos, '.');
        removed += 1;
    }
    part2(grid, removed)
}

//test
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn test_example() {
        let test = Grid::from_str(EXAMPLE);
        assert_eq!(part1(test.clone()), 13);
        assert_eq!(part2(test, 0), 43);
    }
}
