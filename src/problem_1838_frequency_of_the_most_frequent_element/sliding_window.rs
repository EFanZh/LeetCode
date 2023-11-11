pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut start = 0_usize;
        let mut sum = 0;
        let mut result = 0;

        for (i, num) in (1..).zip(&nums) {
            sum += num;

            while num * (i - start) as i32 - sum > k {
                sum -= nums[start];
                start += 1;
            }

            result = result.max(i - start);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_frequency(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
