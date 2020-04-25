struct Solution {}

use std::mem::swap;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums2.len() < nums1.len() {
            swap(&mut nums1, &mut nums2);
        };

        let total_count = nums1.len() + nums2.len();
        let half_total_count = total_count / 2;
        let is_total_count_even = total_count % 2 == 0;

        let mut left = 0;
        let mut size = nums1.len();

        while size > 1 {
            let half_size = size / 2;
            let i = left + half_size;
            let j = half_total_count - i;

            let i_left_value = nums1[i - 1];
            let j_value = nums2[j];

            if i_left_value > j_value {
                size = half_size - 1;
            } else {
                let j_left_value = nums2[j - 1];
                let i_value = nums1[i];

                if j_left_value > i_value {
                    left = i + 1;
                    size -= half_size + 1;
                } else if is_total_count_even {
                    return f64::from(i_left_value.max(j_left_value) + i_value.min(j_value)) / 2.0;
                } else {
                    return f64::from(i_value.min(j_value));
                }
            }
        }

        let mut j = half_total_count - left;

        if size == 1 && nums2[j - 1] > nums1[left] {
            left += 1;
            j -= 1;
        }

        let n = if let Some(&i_value) = nums1.get(left) {
            if let Some(&j_value) = nums2.get(j) {
                i_value.min(j_value)
            } else {
                i_value
            }
        } else {
            nums2[j]
        };

        if is_total_count_even {
            let m = if let Some(&i_left_value) = nums1.get(left.wrapping_sub(1)) {
                if let Some(&j_left_value) = nums2.get((half_total_count - left).wrapping_sub(1)) {
                    i_left_value.max(j_left_value)
                } else {
                    i_left_value
                }
            } else {
                nums2[half_total_count - 1]
            };

            f64::from(m + n) / 2.0
        } else {
            f64::from(n)
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
