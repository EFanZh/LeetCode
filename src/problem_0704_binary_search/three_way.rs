pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut length = nums.len();

        while length != 0 {
            let half = length / 2;
            let middle = start + half;

            match nums[middle].cmp(&target) {
                Ordering::Less => {
                    start = middle + 1;
                    length -= half + 1;
                }
                Ordering::Equal => return middle as _,
                Ordering::Greater => length = half,
            }
        }

        -1
    }
}

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
