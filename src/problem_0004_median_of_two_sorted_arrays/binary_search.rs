pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums2.len() < nums1.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
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

        let n = if let Some(&i_value) = nums1.get(left) {
            if let Some(&j_value) = nums2.get(half_total_count - left) {
                i_value.min(j_value)
            } else {
                i_value
            }
        } else {
            nums2[half_total_count - left]
        };

        if total_count % 2 == 0 {
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
