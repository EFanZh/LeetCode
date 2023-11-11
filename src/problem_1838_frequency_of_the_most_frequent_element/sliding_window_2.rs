pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut start = 0_usize;
        let mut required = 0;
        let mut result = 0;
        let mut prev = 0;

        for (i, &num) in nums.iter().enumerate() {
            required += (num - prev) * (i - start) as i32;

            while required > k {
                required -= num - nums[start];
                start += 1;
            }

            result = result.max(i + 1 - start);

            prev = num;
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
