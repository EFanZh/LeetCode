pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    fn lower_bound<F: FnMut(i32) -> bool>(nums: &[i32], mut left: usize, mut length: usize, mut f: F) -> usize {
        while length > 0 {
            let half = length / 2;
            let middle = left + half;

            if f(nums[middle]) {
                left = middle + 1;
                length -= half + 1;
            } else {
                length = half;
            }
        }

        left
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut length = nums.len();

        while length > 0 {
            let half = length / 2;
            let middle = left + half;

            match nums[middle].cmp(&target) {
                Ordering::Less => {
                    left = middle + 1;
                    length -= half + 1;
                }
                Ordering::Equal => {
                    return vec![
                        Self::lower_bound(&nums, left, half, |x| x < target) as _,
                        (Self::lower_bound(&nums, middle + 1, length - (half + 1), |x| x == target) - 1) as _,
                    ];
                }
                Ordering::Greater => {
                    length = half;
                }
            }
        }

        vec![-1, -1]
    }
}

impl super::Solution for Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::search_range(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
