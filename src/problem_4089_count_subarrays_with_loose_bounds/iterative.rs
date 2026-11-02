pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let min_k = min_k.cast_unsigned();
        let max_k = max_k.cast_unsigned();
        let mut result = 0;
        let mut length = 0;

        for num in nums {
            if num >= min_k && num <= max_k {
                length += 1;
                result += length;
            } else {
                length = 0;
            }
        }

        result
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
