pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut cache = vec![0; nums2.len()];

        for num_1 in nums1 {
            let mut top_left = 0;
            let mut left = 0;

            for (target, &num_2) in cache.iter_mut().zip(&nums2) {
                left = if num_2 == num_1 {
                    top_left + 1
                } else {
                    left.max(*target)
                };

                top_left = mem::replace(target, left);
            }
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_uncrossed_lines(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
