pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied().enumerate();

        loop {
            let (i, num) = iter.next().unwrap();

            for &prev in &nums[i.saturating_sub(3)..i] {
                if num == prev {
                    return num;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn repeated_n_times(nums: Vec<i32>) -> i32 {
        Self::repeated_n_times(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
