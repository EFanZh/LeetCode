pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k as u32;
        let mut cache = (0, 0, 0);

        for num in nums {
            cache = (
                cache.0.min(cache.1).min(cache.2) + u64::from(k.saturating_sub(num)),
                cache.0,
                cache.1,
            );
        }

        cache.0.min(cache.1).min(cache.2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_increment_operations(nums: Vec<i32>, k: i32) -> i64 {
        Self::min_increment_operations(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
