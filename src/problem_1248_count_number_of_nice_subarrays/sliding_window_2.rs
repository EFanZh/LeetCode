pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut required = k as u32;
        let mut result = 0;
        let mut start = 0;
        let mut count = 0;

        for &num in &nums {
            if num % 2 != 0 {
                required -= 1;
                count = 0;
            }

            while required == 0 {
                required += nums[start] as u32 & 1;
                start += 1;
                count += 1;
            }

            result += count;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        Self::number_of_subarrays(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
