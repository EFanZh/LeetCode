pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums_1_sorted = {
            let mut nums_1_sorted = nums1.clone().into_boxed_slice();

            nums_1_sorted.sort_unstable();

            nums_1_sorted
        };

        let mut original_sum = 0;
        let mut max_diff = 0;

        for (&num_1, &num_2) in nums1.iter().zip(&nums2) {
            let original_diff = (num_1 - num_2).abs();

            original_sum += i64::from(original_diff);

            let i = nums_1_sorted.partition_point(|&x| x < num_2);

            if let Some(left) = nums_1_sorted.get(i.wrapping_sub(1)) {
                max_diff = max_diff.max(original_diff - (num_2 - left));
            }

            if let Some(right) = nums_1_sorted.get(i) {
                max_diff = max_diff.max(original_diff - (right - num_2));
            }
        }

        ((original_sum - i64::from(max_diff)) % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::min_absolute_sum_diff(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
