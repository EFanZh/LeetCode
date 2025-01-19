pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut all_bits = 0;
        let mut start = 0;
        let mut i = 0;
        let mut result = 0;

        for &num in &nums {
            while all_bits & num != 0 {
                all_bits ^= nums[start];
                start += 1;
            }

            all_bits |= num;
            i += 1;
            result = result.max(i - start);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        Self::longest_nice_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
