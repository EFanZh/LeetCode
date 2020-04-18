pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let low = nums
            .binary_search_by(|x| if *x < target { Ordering::Less } else { Ordering::Greater })
            .unwrap_err();

        let length = nums[low..]
            .binary_search_by(|x| if target < *x { Ordering::Greater } else { Ordering::Less })
            .unwrap_err();

        if length == 0 {
            vec![-1, -1]
        } else {
            vec![low as _, (low + length - 1) as _]
        }
    }
}

impl super::Solution for Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::search_range(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
