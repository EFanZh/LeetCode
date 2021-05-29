pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut max = i32::min_value();
        let mut right = 0;

        for (i, &num) in nums.iter().enumerate() {
            if num < max {
                right = i + 1;
            }

            max = max.max(num);
        }

        let mut min = i32::max_value();
        let mut left = right;

        for (i, &num) in nums[..right].iter().enumerate().rev() {
            if num > min {
                left = i;
            }

            min = min.min(num);
        }

        (right - left) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        Self::find_unsorted_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
