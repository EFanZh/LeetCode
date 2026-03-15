pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut sum = 0;
        let mut result = 0;
        let mut i = 0;

        while let Some(num) = nums.get_mut(i) {
            sum += *num;

            let num = mem::replace(num, sum);

            result += sum - nums.get(i.wrapping_sub(num as usize + 1)).copied().unwrap_or(0);
            i += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subarray_sum(nums: Vec<i32>) -> i32 {
        Self::subarray_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
