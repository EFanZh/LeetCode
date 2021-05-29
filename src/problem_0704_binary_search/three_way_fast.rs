pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            -1
        } else {
            let mut start = 0;
            let mut length = nums.len();

            while length != 1 {
                let half = length / 2;
                let middle = start + half;

                if nums[middle] <= target {
                    start = middle;
                }

                match nums[middle].cmp(&target) {
                    Ordering::Less => start = middle,
                    Ordering::Equal => return middle as _,
                    Ordering::Greater => {}
                }

                length -= half;
            }

            if nums[start] == target {
                start as _
            } else {
                -1
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::search(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
