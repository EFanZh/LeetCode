pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums2.len() < nums1.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let total_count = nums1.len() + nums2.len();
        let half_total_count_sub_1 = (total_count / 2).wrapping_sub(1);

        let mut left = 0;
        let mut size = nums1.len();

        while size > 1 {
            let half_size = size / 2;
            let i = left + half_size;

            if nums2[half_total_count_sub_1 - i] > nums1[i] {
                left = i;
            }

            size -= half_size;
        }

        let mut j_sub_1 = half_total_count_sub_1 - left;

        if size == 1 && nums2[j_sub_1] > nums1[left] {
            left += 1;
            j_sub_1 = j_sub_1.wrapping_sub(1);
        }

        let j = j_sub_1.wrapping_add(1);
        let n = nums1.get(left).map(Reverse).max(nums2.get(j).map(Reverse)).unwrap().0;

        if total_count % 2 == 0 {
            let m = nums1.get(left.wrapping_sub(1)).max(nums2.get(j_sub_1)).unwrap();

            f64::from(m + n) / 2.0
        } else {
            f64::from(*n)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
