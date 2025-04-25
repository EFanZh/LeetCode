pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let sum = nums.iter().sum::<i32>();
        let mut prefix_sum = 0;

        for value in &mut nums {
            let saved_value = *value;

            *value = (2 * prefix_sum + saved_value - sum).abs();
            prefix_sum += saved_value;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        Self::left_right_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
