pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len() as i32;
        let total_sum = nums.iter().sum::<i32>();
        let mut sum = 0;

        for (t, target) in (2 - n..).step_by(2).zip(&mut nums) {
            sum += *target;

            *target = *target * t + total_sum - sum * 2;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        Self::get_sum_absolute_differences(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
