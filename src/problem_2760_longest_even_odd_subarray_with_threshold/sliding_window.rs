pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut result = 0;
        let mut length = 0;
        let mut prev_is_even = false;

        for num in nums {
            if num <= threshold {
                let is_even = num % 2 == 0;

                if is_even == prev_is_even {
                    result = result.max(length);
                    length = i32::from(is_even);
                } else {
                    length += 1;
                }

                prev_is_even = is_even;
            } else {
                result = result.max(length);
                length = 0;
                prev_is_even = false;
            }
        }

        result.max(length)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        Self::longest_alternating_subarray(nums, threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
