pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut result = 0_u64;
        let mut length = 0;
        let mut prev = 0;

        for num in nums {
            if num == prev {
                result += length * (length + 1) / 2;
                length = 1;
            } else {
                length += 1;
                prev = num;
            }
        }

        result += length * (length + 1) / 2;

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        Self::count_alternating_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
