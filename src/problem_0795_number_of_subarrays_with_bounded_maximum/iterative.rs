pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut result = 0;
        let mut less_than_left = 0;
        let mut less_than_or_equal_to_right = 0;

        for num in nums {
            if num < left {
                less_than_left += 1;
                less_than_or_equal_to_right += 1;

                result += less_than_or_equal_to_right - less_than_left;
            } else if num <= right {
                less_than_left = 0;
                less_than_or_equal_to_right += 1;

                result += less_than_or_equal_to_right;
            } else {
                less_than_left = 0;
                less_than_or_equal_to_right = 0;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        Self::num_subarray_bounded_max(nums, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
