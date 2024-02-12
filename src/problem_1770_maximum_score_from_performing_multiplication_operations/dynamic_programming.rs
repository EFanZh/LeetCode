pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut cache = vec![0; m + 1].into_boxed_slice();

        for (length, &multiplier) in (n - m + 1..).zip(multipliers.iter().rev()) {
            let mut iter = cache[..=n + 1 - length].iter_mut();
            let mut target = iter.next().unwrap();

            for (start, top_right) in iter.enumerate() {
                *target = (*target + nums[start + length - 1] * multiplier).max(*top_right + nums[start] * multiplier);

                target = top_right;
            }
        }

        cache[0]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        Self::maximum_score(nums, multipliers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
