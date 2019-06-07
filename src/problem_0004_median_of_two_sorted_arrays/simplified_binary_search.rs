struct Solution {}

use std::cmp::Reverse;
use std::mem::swap;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums2.len() < nums1.len() {
            swap(&mut nums1, &mut nums2);
        };

        let total_count = nums1.len() + nums2.len();
        let half_total_count = total_count / 2;

        let mut left = 0;
        let mut size = nums1.len();

        while size > 0 {
            let half_size = size / 2;
            let i = left + half_size;

            if nums2[half_total_count - i - 1] > nums1[i] {
                left = i + 1;
                size -= half_size + 1;
            } else {
                size = half_size;
            }
        }

        let j = half_total_count - left;
        let n = nums1.get(left).map(Reverse).max(nums2.get(j).map(Reverse)).unwrap().0;

        if total_count % 2 == 0 {
            let m = nums1
                .get(left.wrapping_sub(1))
                .max(nums2.get((j).wrapping_sub(1)))
                .unwrap();

            f64::from(m + n) / 2.0
        } else {
            f64::from(*n)
        }
    }
}

impl super::Solution for Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Solution::find_median_sorted_arrays(nums1, nums2)
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
