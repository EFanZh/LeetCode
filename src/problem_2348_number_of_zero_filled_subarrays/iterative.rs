pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut length = 0;
        let mut result = 0;

        for &num in &nums {
            if num == 0 {
                length += 1;
                result += length;
            } else {
                length = 0;
            }
        }

        drop(nums);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        Self::zero_filled_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
