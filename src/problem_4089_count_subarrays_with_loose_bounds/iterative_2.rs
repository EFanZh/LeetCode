pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let min_k = min_k.cast_unsigned();
        let max_k = max_k.cast_unsigned();
        let mut result = 0_u64;
        let mut length = 0_u64;

        for num in nums {
            if num >= min_k && num <= max_k {
                length += 1;
            } else {
                result += length * (length + 1) / 2;
                length = 0;
            }
        }

        result += length * (length + 1) / 2;

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        Self::count_subarrays(nums, min_k, max_k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
