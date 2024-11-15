pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut sum_1 = 0;
        let mut sum_2 = 0;
        let mut diff_sum = 0;
        let mut max_diff_sum = 0;
        let mut min_diff_sum = 0;
        let mut max_sub_array_diff = 0;
        let mut min_sub_array_diff = 0;

        nums1.into_iter().zip(nums2).for_each(|(num_1, num_2)| {
            sum_1 += num_1;
            sum_2 += num_2;
            diff_sum += num_1 - num_2;

            max_sub_array_diff = max_sub_array_diff.max(diff_sum - min_diff_sum);
            min_sub_array_diff = min_sub_array_diff.min(diff_sum - max_diff_sum);

            if diff_sum < min_diff_sum {
                min_diff_sum = diff_sum;
            } else if diff_sum > max_diff_sum {
                max_diff_sum = diff_sum;
            }
        });

        (sum_1 - min_sub_array_diff).max(sum_2 + max_sub_array_diff)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::maximums_spliced_array(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
