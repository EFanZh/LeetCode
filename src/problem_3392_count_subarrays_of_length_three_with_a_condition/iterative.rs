pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3)
            .map(|window| i32::from((window[0] + window[2]) * 2 == window[1]))
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_subarrays(nums: Vec<i32>) -> i32 {
        Self::count_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
