pub struct Solution;

use std::cmp::Reverse;
use std::mem;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums2.len() < nums1.len() {
            mem::swap(&mut nums1, &mut nums2);
        };

        let total_count = nums1.len() + nums2.len();
        let half_total_count = total_count / 2;

        let mut left = 0;
        let mut size = nums1.len();

        while size > 1 {
            let half_size = size / 2;
            let i = left + half_size;
            let j = half_total_count - i;

            if nums1[i - 1] > nums2[j] {
                size = half_size - 1;
            } else if nums2[j - 1] > nums1[i] {
                left = i + 1;
                size -= half_size + 1;
            } else {
                left = i;
                size = 0;
            }
        }

        let mut j = half_total_count - left;

        if size == 1 && nums2[j - 1] > nums1[left] {
            left += 1;
            j -= 1;
        }

        let n = nums1.get(left).map(Reverse).max(nums2.get(j).map(Reverse)).unwrap().0;

        if total_count % 2 == 0 {
            let m = nums1
                .get(left.wrapping_sub(1))
                .max(nums2.get(j.wrapping_sub(1)))
                .unwrap();

            f64::from(m + n) / 2.0
        } else {
            f64::from(*n)
        }
    }
}

impl super::Solution for Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Self::find_median_sorted_arrays(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
