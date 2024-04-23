pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().fold(0_i64, |sum, &x| sum + i64::from(x));
        let nums = &nums[..nums.len() - 1];
        let mut left_sum = 0;
        let mut result = 0;

        for &num in nums {
            left_sum += i64::from(num);
            result += i32::from(left_sum + left_sum >= sum);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        Self::ways_to_split_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
