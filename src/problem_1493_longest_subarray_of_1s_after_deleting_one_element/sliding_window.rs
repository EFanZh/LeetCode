pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut length = 0;
        let mut zeros = 0_u32;

        for &num in &nums {
            zeros += u32::from(num == 0);

            if zeros < 2 {
                length += 1;
            } else {
                zeros -= u32::from(nums[start] == 0);
                start += 1;
            }
        }

        length - 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subarray(nums: Vec<i32>) -> i32 {
        Self::longest_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
