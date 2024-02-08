pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;
        let ones = nums.iter().sum::<i32>() as u32;

        let (left, right) = nums.split_at(ones as usize);
        let mut window_ones = left.iter().sum::<i32>() as u32;
        let mut max_windows_ones = window_ones;

        for (&old, &new) in nums.iter().zip(right) {
            window_ones -= old as u32;
            window_ones += new as u32;
            max_windows_ones = max_windows_ones.max(window_ones);
        }

        let (left, right) = nums.split_at((n - ones) as usize);
        let mut window_ones = left.iter().sum::<i32>() as u32;
        let mut min_windows_ones = window_ones;

        for (&old, &new) in nums.iter().zip(right) {
            window_ones -= old as u32;
            window_ones += new as u32;
            min_windows_ones = min_windows_ones.min(window_ones);
        }

        (ones - max_windows_ones).min(min_windows_ones) as _
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
