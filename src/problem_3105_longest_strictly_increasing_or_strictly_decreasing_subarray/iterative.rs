pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0_u32;
        let mut prev = 0;
        let mut decreasing_length = 0;
        let mut increasing_length = 0;

        for num in nums {
            let num = num as u8;

            if num < prev {
                decreasing_length += 1;
            } else {
                decreasing_length = 1;
            }

            if num > prev {
                increasing_length += 1;
            } else {
                increasing_length = 1;
            }

            result = result.max(decreasing_length).max(increasing_length);
            prev = num;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        Self::longest_monotonic_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
