pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.first()
            .and_then(|first| {
                if target < *first {
                    nums.binary_search_by(|x| if x < first { x.cmp(&target) } else { Ordering::Less })
                } else {
                    nums.binary_search_by(|x| if x < first { Ordering::Greater } else { x.cmp(&target) })
                }
                .ok()
            })
            .map_or(-1, |i| i as _)
    }
}

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search(nums, target)
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
