pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::iter;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut occupied = HashSet::<_>::from_iter(nums);

        iter::zip(move_from, move_to).for_each(|(from, to)| {
            if occupied.remove(&from) {
                occupied.insert(to);
            }
        });

        let mut result = Vec::from_iter(occupied);

        result.sort_unstable();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        Self::relocate_marbles(nums, move_from, move_to)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
