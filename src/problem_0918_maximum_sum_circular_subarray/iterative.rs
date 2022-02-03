pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut min_result = i32::MAX;
        let mut max_result = i32::MIN;
        let mut sum = 0;
        let mut min_sum = 0;
        let mut max_sum = 0;

        for num in nums {
            min_result = min_result.min(sum - max_sum);

            sum += num;

            max_result = max_result.max(sum - min_sum);
            min_sum = min_sum.min(sum);
            max_sum = max_sum.max(sum);
        }

        max_result.max(sum - min_result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        Self::max_subarray_sum_circular(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
