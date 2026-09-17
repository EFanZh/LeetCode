pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        let iter = nums.iter().zip(&target);
        let mut set = HashSet::with_capacity(iter.len());

        set.extend(
            nums.iter()
                .zip(&target)
                .filter_map(|(&num, &target)| (num != target).then_some(num)),
        );

        set.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>, target: Vec<i32>) -> i32 {
        Self::min_operations(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
