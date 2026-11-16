pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = nums.len() as u32;
        let ones = nums.iter().sum::<u32>();

        let (left, right) = nums.split_at(ones as usize);
        let mut window_ones = left.iter().sum::<u32>();
        let mut max_windows_ones = window_ones;

        for (&old, &new) in nums.iter().zip(right) {
            window_ones -= old;
            window_ones += new;
            max_windows_ones = max_windows_ones.max(window_ones);
        }

        let (left, right) = nums.split_at((n - ones) as usize);
        let mut window_ones = left.iter().sum::<u32>();
        let mut min_windows_ones = window_ones;

        for (&old, &new) in nums.iter().zip(right) {
            window_ones -= old;
            window_ones += new;
            min_windows_ones = min_windows_ones.min(window_ones);
        }

        (ones - max_windows_ones).min(min_windows_ones).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps(nums: Vec<i32>) -> i32 {
        Self::min_swaps(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
